// k6 load test, run by ./performance_test.sh (docker-compose-performance.yml).
//
// Built on the shared OpenAPI coverage module (mairie360/CICD `tests/k6/coverage.js`, MAIR-194):
// `createCoverage` needs exactly one handler per operation of the spec served by the API
// ("METHOD /path", path as in openapi.json) and k6 aborts at init otherwise. When you add an
// endpoint, add its handler here and send its request through `request()` (raw `http.*` calls
// are not counted). Operations needing data (an existing id, a valid body) get it from setup()
// or from init-test.sql.
import { check, sleep } from 'k6';
import { createCoverage } from '/coverage.js';

// Static HS256 JWT (sub=1, the Admin seeded by liquibase, role=admin, exp=2100, signed with the
// stack's JWT_SECRET=b"secret"), the same one ZAP injects. Sent on every request so the
// `bearer_auth` operations under /api are exercised authenticated; public routes ignore it.
const TOKEN =
  __ENV.JWT ||
  'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxIiwicm9sZSI6ImFkbWluIiwiZXhwIjo0MTAyNDQ0ODAwfQ.xCeBe_2QxRlXW8WXr3t6F69wbEHA93HbP_7l4OTJwjA';

const coverage = createCoverage({
  'GET /health': ({ request }) =>
    check(request(), {
      'health 200': (r) => r.status === 200,
    }),
  'POST /': ({ request }) =>
    check(request(), {
      'hello 200': (r) => r.status === 200,
    }),
});

export const options = {
  stages: [
    { duration: '30s', target: 20 }, // Ramp up to 20 virtual users
    { duration: '1m', target: 20 }, // Hold
    { duration: '10s', target: 0 }, // Ramp down
  ],
  thresholds: {
    ...coverage.thresholds, // every operation exercised, no handler error
    http_req_duration: ['p(95)<200'], // 95% of the requests under 200ms on the reference machine
    http_req_failed: ['rate<0.01'], // Less than 1% errors
  },
};

export default function () {
  coverage.run({ headers: { Authorization: `Bearer ${TOKEN}` } });
  sleep(1);
}
