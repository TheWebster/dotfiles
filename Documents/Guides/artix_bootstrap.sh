#!/bin/sh

### CONFIGURATION ###
DRIVE=
VG_NAME=system
ROOT_SIZE=
VAR_SIZE=
HOME_SIZE=
SWAP_SIZE=

REGION=Europe
CITY=Berlin
LOCALES="en_US.UTF8 de_DE.UTF8"

HOSTNAME=
USERNAME=

### GLOBALS ###
_LVS="root var home swap"

### FUNCTIONS ###
function check_var() {
  var=${!$1}
  
  if [ -z $var ]
  then
    echo "Error: \"$1\" not set"
  fi
}

### CHECK CONFIGURATION ###
check_var DRIVE
check_var VG_NAME
check_var REGION
check_var CITY
check_var HOSTNAME
check_var USERNAME

for lv in $_LVS
do
  lv_size=${lv^^}_SIZE
  check_var $lv_size
done

### SETUP PARTITIONS ###
parted --script --align optimal ${DRIVE} -- \
  mklabel gpt \
  mkpart 'EFI' fat32 0 512MiB \
  mkpart 'SYSTEM-LVM' 512MiB -1s \

_PART_EFI=/dev/disk/by-partlabel/EFI
_PART_LVM=/dev/disk/by-partkabel/SYSTEM-LVM

### SETUP LVM ###
pvcreate ${_PART_LVM}
vgcreate system ${_PART_LVM}
for lv in $_LVS
do
  size=${lv^^}_SIZE
  lvcreate -L $size -n $lv ${VG_NAME}
done

_PART_ROOT=/dev/system/root
_PART_VAR=/dev/system/var
_PART_HOME=/dev/system/home
_PART_SWAP=/dev/system/swap

### CREATE FILESYSTEMS ###
mkfs.fat -F32 ${_PART_EFI}
fatlabel ${_PART_EFI} efi
for lv in $_LVS
do
  [ $lv != swap ] && mkfs.ext4 -L $lv /dev/system/${lv}
done 

mkswap -L swap /dev/system/swap

### MOUNT FILESYSTEMS ###
mount /dev/system/root /mnt
cd /mnt
mkdir -p boot/efi var home
mount /dev/system/var/ var
mount /dev/system/home home
mount ${_PART_EFI} /boot/efi
swapon /dev/system/swap
cd -

### SET CLOCK ###

### INSTALL BASE SYSTEM ###
basestrap /mnt \
  base \
  base-devel \
  s6-base \
  elogind-s6 \
  lvm2 \
  connman-s6 \
  linux \
  linux-firmware \
  helix \
  grub \
  os-prober \
  efibootmgr \
  xorg-server \
  rustup \
  fish \
  artix-archlinux-support

fstabgen -U /mnt >> /mnt/etc/fstab

### SETUP SYSTEM ###
artix-chroot /mnt

ln -sf /usr/share/zoneinfo/${REGION}/${CITY} /etc/localtime

for loc in $LOCALES
do
  cat /etc/locale.gen | sed -r "s/^#(${loc})/\1/g" > /etc/locale.gen
done
locale-gen

echo 'LANG="en_US.UTF8"' >> /etc/locale.conf
echo "KEYMAP=de-latin1-nodeadkeys" >> /etc/vconsole.conf

grub-install --target=x86_64-efi --efi-directory=/boot/efi --bootloader-id=grub
grub-mkconfig -o /boot/grub/grub.cfg

cat /etc/mkinitcpio.conf | sed -r 's/^(HOOKS=.*block)(.*)$/\1 lvm2\2/g' > /etc/mkinitcpio.conf
mkinitcpio -P

cat << EOF >> /etc/hostname
127.0.0.1 localhost
::1       localhost
127.0.0.1 ${HOSTNAME}.localdomain ${HOSTNAME}
EOF

touch /etc/s6/adminsv/default/contents.d/connmand
s6-db-reload

passwd
useradd -m $USERNAME
passwd $USERNAME

cat << EOF >> /etc/pacman.conf
#Arch
[extra]
Include = /etc/pacman.d/mirrorlist-arch
EOF

