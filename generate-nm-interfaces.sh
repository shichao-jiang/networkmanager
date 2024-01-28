#!/usr/bin/env bash
set -e

readonly NETWORKMANAGER_RELEASE="1.42.2"

if !which git &> /dev/null; then
  echo "Git must be installed!!"
  exit 1
fi

root="$(readlink -f "$(dirname "$0")")"
if [[ ! -d "$root" ]]; then
  echo "Could not find root $root"
  exit 1
fi

mkdir -p $root/tmp
git clone --depth 1 https://gitlab.freedesktop.org/NetworkManager/NetworkManager.git $root/tmp/
cd tmp
git fetch --depth 1 origin tags/$NETWORKMANAGER_RELEASE
git switch -d FETCH_HEAD
cd ..

if ! hash dbus-codegen-rust 2> /dev/null; then
  echo "Could not find dbus-codegen-rust binary. Do you want to install it using Cargo?"
  echo -n "[Yn] > "
  read -r c
  if [[ $c == "y" || $c == "Y" ]]; then
    cargo install dbus-codegen
  else
    exit 1
  fi
fi

dest="$root/src/genz"
mkdir -p $dest

alltraits=()
allmods=()
for spec in $root/tmp/introspection/*.xml; do
  basename=$(basename "$spec" .xml)
  trait=( $(IFS=. ; printf '%s ' $basename) )
  trait=$(echo ${trait[@]^} | tr -d "[:space:]")
  trait=${trait#OrgFreedesktopNetworkManager}
  [[ -z $trait ]] && trait="NetworkManager"
  alltraits+=($trait)
  modname=${basename#org.freedesktop.NetworkManager}
  modname=nm${modname//./_}
  modname=$(echo $modname | tr '[:upper:]' '[:lower:]')
  allmods+=($modname)
  dest_file="${dest}/${modname}".rs
  echo "Generating code from $(basename "${spec}") to ${modname}…"
  zbus-xmlgen "$spec" | sed -E "s/^trait \w+/pub trait $trait/" > "$dest_file"
done

echo "#![allow(warnings, rust_2018_idioms)]" > $dest/mod.rs

for mod in ${allmods[@]}; do
  echo "mod "$mod";" >> $dest/mod.rs
done

echo -e "\n" >> $dest/mod.rs

for ((i=0;i<${#allmods[@]};++i)); do
  echo "pub(super) use ${allmods[i]}::{${alltraits[i]}Proxy, ${alltraits[i]}ProxyBlocking};" >> $dest/mod.rs
done

echo "Formatting code... "
rustfmt $dest/*.rs
echo "Done."

echo "Cleaning up..."
rm -rf $root/tmp
echo "Done."