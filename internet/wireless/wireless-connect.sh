#!/bin/bash

wpa_supplicant -B -d -i wlp1s0 -Dnl80211 -c/home/aidan/.config/internet/wireless/wpa_supplicant.conf
dhcpcd wlp1s0
