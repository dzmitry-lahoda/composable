#!/bin/bash
#
# check for any changes in the runtime/ and frame/*. if
# there are any changes found, it should mark the PR breaksconsensus and
# "auto-fail" the PR if there isn't a change in the runtime/src/lib.rs file
# that alters the version.

#set -e # fail on any error

#shellcheck source=../common/lib.sh
. "$(dirname "${0}")/./common/lib.sh"

# shellcheck disable=SC2039
VERSIONS_FILES=(
  "runtime/picasso/src/lib.rs,picasso,picasso"
  "runtime/dali/src/lib.rs,dali-chachacha,dali"
   "runtime/composable/src/lib.rs,composable,composable"
)

 echo "latest 10 commits of ${GITHUB_REF_NAME}"
 git log --graph --oneline --decorate=short -n 10

 echo "make sure the main branch and release tag are available in shallow clones"
 git fetch --depth="${GIT_DEPTH:-100}" origin "${BASE_BRANCH}"
 

simnode_check() {
  VERSIONS_FILE="$1"
  if has_runtime_changes "${BASE_BRANCH}" "${GITHUB_REF_NAME}" "$3" && check_runtime "$VERSIONS_FILE" "$2"
  then
    echo "Wasm sources have changed"
    echo "RUNTIME_CHECK=1" >> "$GITHUB_ENV"
  fi
}

for i in "${VERSIONS_FILES[@]}"; do
  while IFS=',' read -r output chain folder; do
    echo "check if the wasm sources changed for $chain"
    simnode_check $output $folder
  done <<< "$i"
done

# dropped through. there's something wrong;  exit 1.
exit 0

# vim: noexpandtab
