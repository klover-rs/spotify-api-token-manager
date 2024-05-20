#!/bin/bash

json_data='{"refresh_token":"AQAJ4nNhY5tvCpMIE-5xs4eBtuXShijVcwjwFUgBvIO6BPmtP9ddjDkXgA0yZY1sdzO6oJzKgMzSHrF46UqDDF7pVzQUU-FWylx_eDQH9OJmy2RiebXYg2nScGfqnOq4R7o"}'

curl -X POST \
  -H "Content-Type: application/json" \
  -d "$json_data" \
  http://127.0.0.1:8080/refreshToken
