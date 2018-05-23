#!/bin/bash
# moves important config files to /home/aidan/config and sets up symbolic links that point to them
if [ `hostname` == jade-sky ] 
then
	HOST="dell"	
elif [ `hostname` == sky-xul ]
then
	HOST="chromebook"
fi


# pacman config file
rm /etc/pacman.conf 
ln -sf /home/aidan/.config/pacman/pacman.conf /etc/pacman.conf

# pacman mirrorlist
rm /etc/pacman.d/mirrorlist 
ln -sf /home/aidan/.config/pacman/mirrorlist /etc/pacman.d/mirrorlist

# X11 keyboard file
cp /home/aidan/.config/keyboard/us-$HOST /usr/share/X11/xkb/symbols/us

#xorg keyboard conf file
cp /home/aidan/.config/keyboard/10-keyboard.conf /etc/X11/xorg.conf.d

#virtual console keymap file
cp /home/aidan/.config/keyboard/dvp.map.gz-$HOST /usr/share/kbd/keymaps/i386/dvorak/dvp.map.gz

# vconsole
rm -f /etc/vconsole.conf
ln -s /home/aidan/.config/vconsole.conf /etc/vconsole.conf

#chromebook touchpad file
if [[ $HOST == "chromebook" ]] 
then  
	ln -sf /home/aidan/.config/x/50-cros-touchpad.conf /etc/X11/xorg.conf.d/
fi

#other xorg config files
ln -sf /home/aidan/.config/x/xauthority /home/aidan/.Xauthority
ln -sf /home/aidan/.config/x/xresources-$HOST /home/aidan/.Xresources
ln -sf /home/aidan/.config/x/xbindkeysrc-$HOST /home/aidan/.xbindkeysrc
ln -sf /home/aidan/.config/x/xinitrc /home/aidan/.xinitrc

#bash stuff
rm /home/aidan/.bashrc
ln -sf /home/aidan/.config/bash/bashrc /home/aidan/.bashrc
ln -sf /home/aidan/.config/bash/bash_profile /home/aidan/.bash_profile

#cmus 
ln -sf /home/aidan/.config/cmus /home/aidan/.cmus

#vim 
ln -sf /home/aidan/.config/vim /home/aidan/.vim


# top config file
ln -sf /home/aidan/.config/toprc /home/aidan/.toprc

#offlineimap
ln -sf /home/aidan/.config/offlineimap /home/aidan/.offlineimap
ln -sf /home/aidan/.config/offlineimap/offlineimaprc /home/aidan/.offlineimaprc
mkdir -p /home/aidan/mail/Gmail
ln -sf /home/aidan/.config/netrc /home/aidan/.netrc
chmod 600 /home/aidan/.netrc


# weechat
ln -sf /home/aidan/.config/weechat /home/aidan/.weechat

# mutt
ln -sf /home/aidan/.config/mutt /home/aidan/.mutt
ln -sf /home/aidan/.config/mutt/muttrc /home/aidan/.muttrc
ln -sf /home/aidan/.config/mutt/urlview /home/aidan/.urlview

# mailcap file
 ln -sf /home/aidan/.config/mutt/mailcap /etc/mailcap

#xmonad 
ln -sf /home/aidan/.config/xmonad/$HOST /home/aidan/.xmonad
	
#ssh
ln -sf /home/aidan/.config/ssh/$HOST /home/aidan/.ssh


#audio for chromebook
if [[ $HOST == "chromebook" ]]
then 
	cp /home/aidan/.config/audio/etc-modprobe-alsa.conf-chromebook /etc/modprobe.d/alsa.conf
fi

# luakit
mkdir -p /home/aidan/.local/share
rm -r -f /home/aidan/.local/share/luakit
ln -sf /home/aidan/.config/luakit/local-share /home/aidan/.local/share/luakit

rm -r /usr/share/luakit
ln -sf /home/aidan/.config/luakit/usr-share /usr/share/luakit

#makepkg config file
ln -sf /home/aidan/.config/makepkg/makepkg.conf-$HOST /etc/makepkg.conf 

#change permissions and ownership of /home/aidan
chown -R aidan /home/aidan
chmod -R a+rwx /home/aidan

# rtorrent 
ln -s /home/aidan/.config/rtorrent/rtorrent.rc /home/aidan/.rtorrent.rc
mkdir /home/aidan/.rtorrent-session
