default:
  just --list

check_semver:
  cd 01.workspace/heave && cargo semver-checks --baseline-rev origin/next --verbose

extract_devlog date:
  git log --pretty=format:"%ad|%s" --date=short --since={{date}} | extract-devlog.sh

sync_remotes:
  git push codeberg.org next
  git push github.com next
