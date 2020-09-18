#!/bin/bash
curl -i -d "{\"name\":\"Shanny5\",\"breed\":\"Bernese mountain dog\", \"age\":10}" -X POST https://wtab0wx9z6.execute-api.eu-north-1.amazonaws.com/dev/puppy -H "content-type: application/json"

curl -i https://wtab0wx9z6.execute-api.eu-north-1.amazonaws.com/dev/puppy/Shanny