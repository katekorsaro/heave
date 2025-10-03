default:
  just --list

sync_codeberg:
  git push -u codeberg.org next
