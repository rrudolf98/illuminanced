#!/bin/sh

DEST_BIN='/usr/sbin'
DEST_ETC='/etc/illuminanced'
ROOT=`dirname $0`

if [ -e /etc/systemd/system/illuminanced.service ]; then
    echo -n "Try stop service: "
    systemctl stop illuminanced.service || echo " Failed"
    echo " Done"
fi

if [ ! -d $DEST_ETC ]; then
    mkdir $DEST_ETC || exit "Cannot create $DEST_ETC"
fi

if [ ! -d $DEST_BIN ]; then
    mkdir $DEST_BIN || exit "Cannot create $DEST_BIN"
fi

echo "\nInstall:"

cp -v $ROOT/target/release/illuminanced $DEST_BIN || exit 1
cp -v $ROOT/illuminanced.conf $DEST_ETC  || exit 1
cp -v illuminanced.service /etc/systemd/system/illuminanced.service || exit 1

echo "\nStart service:"

systemctl enable illuminanced.service || exit 1
systemctl start illuminanced.service || exit 1
systemctl status illuminanced.service || exit 1
