in order to hide grub unless you hold down shift
	add to /etc/default/grub: GRUB_FORCE_HIDDEN_MENU="true"
	copy 31_hold_shift to /etc/grub.d/ and make executable
	regenerate grub config with grub-mkconfig -o /boot/grub/grub.cfg
