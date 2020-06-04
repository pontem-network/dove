set -e
set -x

cd ./resources/grammar
tree-sitter generate
rm binding.gyp index.js src/binding.cc src/grammar.json src/node-types.json
cd -