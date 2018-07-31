rm -rf dist_prod \
  && npm run build -- release \
  && webpack \
  && cp static/lib.js dist_prod/ \
  && cp static/index.html dist_prod/ \
  && aws s3 sync dist_prod/ s3://rb-smithy-todo-list/ --cache-control max-age=0,no-cache --delete \
  && aws s3 cp dist_prod/*.wasm s3://rb-smithy-todo-list/ \
    --cache-control max-age=0,no-cache \
    --content-type application/wasm \
  && aws cloudfront create-invalidation --distribution-id E3IDF5NLG30OGP --paths '/*'
