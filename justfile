default:
  just --list

sync_remotes:
  git push -u codeberg.org next
  git push -u github.com next
