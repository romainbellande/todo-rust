#!/usr/bin/env bash

rm -rf ./dist;

trunk build --release;

release_filename=`ls ./dist | grep wasm`;

no_opt_size=`du -sh ./dist/${release_filename} | xargs | cut -d " " -f1`;

prefix=`echo ${release_filename} | cut -d "-" -f1`;
version=`echo ${release_filename} | cut -d "-" -f2 | cut -d "_" -f1`;
suffix=`echo ${release_filename} | cut -d "_" -f2`;

release_opt_filename="${prefix}-${version}_opt_${suffix}";

wasm-opt "./dist/${release_filename}" -Os -o "./dist/${release_opt_filename}";

no_opt_size=`du -sh ./dist/${release_filename} | xargs | cut -d " " -f1`;

opt_size=`du -sh ./dist/${release_opt_filename} | xargs | cut -d " " -f1`;

echo "before optimization: ${no_opt_size}, after optimization: ${opt_size}";
