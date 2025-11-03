default:
  just --list

extract_devlog date:
  git log --pretty=format:"%ad|%s" --date=short --since="{{date}} 00:00" | extract-devlog.sh

check_semver:
  cd 01.workspace/heave && cargo semver-checks --baseline-rev origin/next --verbose

code_coverage:
  cd 01.workspace && cargo llvm-cov clean && cargo llvm-cov --show-missing-lines

sync_remotes:
  git push codeberg.org next
  git push github.com next
