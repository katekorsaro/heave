default:
  just --list

check_semver:
  cd 01.workspace/heave && cargo semver-checks --baseline-rev origin/next

sync_remotes:
  git push codeberg.org next
  git push github.com next
