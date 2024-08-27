
- set `/etc/conf.d/keymaps` and `/etc/vconsole.conf`
- partition disk:
  
  |Role|Filesystem|Size  |
  |----|----------|------|
  |ESP |fat32     |512MiB|
  |LVM |lvm       |*rest*|

- setup LVM

  |Name|Size |
  |----|-----|
  |root|50GiB|
  |var |50GiB|
  |home|50GiB|
  |swap|16GiB|

- format and mount drives to `/mnt`
  ```bash
  $ mkfs.fat -F32 ${drive}
  $ mkfs.ext4 ${drive}
  $ mkswap ${drive}
  ```
  ESP mounts to /boot/efi
  
- update system clock  
  `$ s6-rc -u change ntpd`

- install base system  
  ```bash
  $ basestrap /mnt base base-devel s6-base elogind-s6 lvm2 connman-s6 linux\
                   linux-firmware helix grub os-prober efibootmgr
  ```

- generate fstab  
  `$ fstabgen -U /mnt >> /mnt/etc/fstab`

- chroot into new system  
  `$ artix-chroot /mnt`
  
- set timezone  
  `$ ln -sf /usr/share/zoneinfo/Region/City /etc/loacltime`

- set hwclock  
  `$ hwclock --systohc`

- set locale
  ```bash
  $ helix /etc/locale.gen
  $ locale-gen
  $ helix /etc/locale.conf
    export LANG="en_US.UTF8"
  ````

- setup grub
  ```bash
  $ grub-install --target=x86_64-efi --efi-directory=/boot/efi --bootloader-id=grub
  $ grub-mkconfig -o /boot/grub/grub.cfg
  ```

- Add users
  ```bash
  $ passwd
  $ useradd -m user
  $ passwd user
  ```

- Networking
  ```bash
  echo ${hostname} >> /etc/hostname
  echo "
    127.0.0.1   localhost
    ::1         localhost
    127.0.0.1   hostname.localdomain hostname
  " >> /etc/hosts
  ```

- Pray
