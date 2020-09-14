#!/bin/bash

cd $(dirname $0)

mkdir -p /Library/Google/Chrome/NativeMessagingHosts
cp com.iota.stronghold.json /Library/Google/Chrome/NativeMessagingHosts
