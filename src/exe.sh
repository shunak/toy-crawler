#!/bin/bash
./toy-crawler > output_tmp.json
cat output_tmp.json | sed -e s'#\\##g' | sed -e 's/\"{/{/g' | sed -e 's/}\"/}/g' > output.json
gsutil cp /Users/syu/toy-crawler/src/output.json gs://testhosting202209
gsutil setmeta -h "Cache-Control: no-store" gs://testhosting202209/output.json
rm output_tmp.json
