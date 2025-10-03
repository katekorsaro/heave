default:
  just --list

sync_remotes:
  git push codeberg.org next
  git push github.com next
