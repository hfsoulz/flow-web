author: Andreas
published: 2022-01-13 21:14:00
updated: 2022-01-13 21:14:00
topics: Lenovo Thinkpad X200, coreboot, flashrom, Raspberry Pi, ch341a
title: Flashing coreboot on a Lenovo Thinkpad X200 with a Raspberry Pi Tutorial
snippet: In this tutorial, we will go through the steps to get coreboot compiled and flashed on a Lenovo Thinkpad X200 laptop.

---

In this tutorial, we will go through the steps to get [coreboot](https://www.coreboot.org/) compiled and flashed on a **Lenovo Thinkpad X200** laptop.

We'll also change the Wifi card so that Wifi will work with a libre kernel driver.

# Prerequisites

![Prerequisites](/static/blog/media/x200/01-prerequisites.jpg)

Things you need:

- Lenovo Thinkpad X200
- Atheros AR9382 AR5BHB116 Mini PCIE Dual Band Wifi card
- Raspberry Pi or ch341a USB programmer
- Pomona 5252 SOIC-16 clip
- 6 x 10 cm female - female jumper wires
- Screwdriver

This tutorial assumes you're using a GNU/Linux distro and are familiar
with how to use a terminal.

[Debian GNU/Linux](https://www.debian.org/) 11 (bullseye) was used to
compile the coreboot image produced in this tutorial.

Files needed:

[`dejavusansmono.pf2`](/static/blog/media/x200/dejavusansmono.pf2)  
[`grub.cfg`](/static/blog//media/x200/grub.cfg)  
[`background.jpg`](/static/blog/media/x200/background.jpg)

For more information about DejaVu Fonts, see:  
[https://dejavu-fonts.github.io/](https://dejavu-fonts.github.io/)

For DejaVu Font license, see:  
[https://dejavu-fonts.github.io/License.html](https://dejavu-fonts.github.io/License.html)

'**background.jpg**' is borrowed from the libreboot project and it's license
information is as follows:

```plaintext
These images are based on http://libreboot.org/logo/logo.svg
which is Copyright 2014 Marcus Moeller and released under CC-0
https://creativecommons.org/publicdomain/zero/1.0/legalcode
```

# Packages needed to compile flashrom/coreboot/grub

```sh
sudo apt-get build-dep flashrom grub
sudo apt-get install git build-essential gnat flex bison libncurses5-dev libfreetype-dev pkg-config unifont wget
```

# Compile flashrom

Clone flashrom git repository:

```sh
git clone https://review.coreboot.org/flashrom.git
```

Compile flashrom:

```sh
cd flashrom
sudo make install
sudo cp /usr/local/sbin/flashrom /usr/local/bin/
```

# Compile GRUB

See '**Prerequisites**' above for the files needed.

Clone GRUB git repository:

```sh
git clone https://git.savannah.gnu.org/git/grub.git
```

Compile GRUB:

```sh
cd grub
./bootstrap
./configure --with-platform=coreboot
make -j4
```

Create '**pack-payload-minimal.sh**' script file:

```sh
touch pack-payload-minimal.sh
chmod +x pack-payload-minimal.sh
```

Paste the following in '**pack-payload-minimal.sh**':

```sh
#! /bin/bash

# ./pack-payload-minimal.sh – To be invoked within GRUB project folder, packs
# an executable elf payload file for the coreboot project, using just one
# keyboard layout file. Adapt “settings” to your needs. Specify a lighter
# pf2-fontfile if available on your system.

# settings
elf_output="grub2.elf"
#pf2_fontfile="unicode.pf2"
pf2_fontfile="dejavusansmono.pf2"
keyboard_layout="se"
grub_modules="cbfs"

# generate keyboard layout
ckbcomp "${keyboard_layout}" | ./grub-mklayout -o "${keyboard_layout}.gkb"

# pack the GRUB payload file
./grub-mkstandalone \
        --grub-mkimage=./grub-mkimage \
        -O i386-coreboot \
        -o "${elf_output}" \
        -d grub-core \
        --fonts= \
        --themes= \
        --locales= \
        --modules="${grub_modules}" \
        /boot/grub/grub.cfg=coreboot.cfg \
        /boot/grub/fonts/${pf2_fontfile}="${pf2_fontfile}" \
        /boot/grub/layouts/${keyboard_layout}.gkb="${keyboard_layout}.gkb"

# message
echo "Payload ${elf_output} has been packed with:"
echo " * fontfile: ${pf2_fontfile}"
echo " * keyboard layout: ${keyboard_layout}"
echo " * GRUB modules, to be preloaded: ${grub_modules}"

# code snippet suggestion
echo "Don't forget to add 'keymap ${keyboard_layout}' to your GRUB Configuration File."

# clean up
rm "${keyboard_layout}.gkb"
```

Execute the script to create '**grub2.elf**' file:

```sh
./pack-payload-minimal.sh
```

# Connect programmer to flash chip

See [`Setup Raspberry Pi for flashing with flashrom Tutorial`](/blog/setup-raspberry-pi-for-flashing-with-flashrom-tutorial/)
if you're using a Raspberry Pi.

Remove the following parts to access the flash chip:

```plaintext
Battery
Keyboard
Palmrest
```

![Connect programmer](/static/blog/media/x200/02-lenovo-thinkpad-x200-internals.jpg)

Connect Pomona 5252 to flash chip like this:

```plaintext
Screen (furthest from you)
             __
   N/C  1 --|  |-- 9   MISO
   GND  2 --|  |-- 10  CS
   N/C  3 --|  |-- 11  N/C
   N/C  4 --|  |-- 12  N/C
   N/C  5 --|  |-- 13  N/C
   N/C  6 --|  |-- 14  N/C
  MOSI  7 --|  |-- 15  3.3V
   CLK  8 --|__|-- 16  N/C

   N/C = Not connected / Not used

   Edge (closest to you)
```

![ch341a USB programmer connected](/static/blog/media/x200/03-raspberry-pi-connected.jpg)

# Read Factory BIOS

Read Factory BIOS 3 times if using **Raspberry Pi**:

```sh
sudo flashrom -p linux_spi:dev=/dev/spidev0.0,spispeed=32768 -r factory_bios_01.rom -V
sudo flashrom -p linux_spi:dev=/dev/spidev0.0,spispeed=32768 -r factory_bios_02.rom -V
sudo flashrom -p linux_spi:dev=/dev/spidev0.0,spispeed=32768 -r factory_bios_03.rom -V
```

Read Factory BIOS 3 times if using **ch341a**:

```sh
sudo flashrom -p ch341a_spi -r factory_bios_01.rom -V
sudo flashrom -p ch341a_spi -r factory_bios_02.rom -V
sudo flashrom -p ch341a_spi -r factory_bios_03.rom -V
```

Make sure checksums are identical:

```sh
sha512sum *.rom
```

# Download coreboot

Download coreboot:

```sh
wget https://coreboot.org/releases/coreboot-4.15.tar.xz
wget https://coreboot.org/releases/coreboot-blobs-4.15.tar.xz
```

**coreboot-blobs-4.15.tar.xz** is needed for CPU microcode updates.

Extract coreboot and blobs needed:

```sh
tar xvf coreboot-4.15.tar.xz
tar xvf coreboot-blobs-4.15.tar.xz --strip-components=1 -C coreboot-4.15
```

# Setup and compile coreboot

Enter coreboot folder:

```sh
cd coreboot-4.15
```

Build toolchain needed first (this will take a long time, be patient):

```sh
make crossgcc-i386 CPUS=4
```

coreboot settings menu:

```sh
make menuconfig
```

Set the following options:

````plaintext
NOTE: Leave default values as is and specifically set the following
options:

mainboard -|
           |-Mainboard vendor (Lenovo)
           |-Mainboard model (ThinkPad X200)
           |-ROM chip size (8192 KB (8 MB))
           |-(0x300000) Size of CBFS filesystem in ROM
chipset ---|
           |-Include CPU microcode in CBFS (Generate from Tree)
Devices ---|
           |-Graphics initialization (Use libgfxinit)
           |-Display
           |-    Framebuffer mode (Linear "high-resolution" framebuffer)
Generic Dr.|
           |- Select: PS/2 keyboard init
payload ---|
           |-Add a payload (An ELF executable payload)
           |-"(/path/to/grub/grub2.elf)" Payload path and filename

If you don't want to include CPU microcode updates:

```plaintext
chipset ---|
           |-Include CPU microcode in CBFS (Do not include microcode updates)
````

This will create a '**.config**' file containing all settings.

Compile coreboot:

```sh
make -j4
```

This will create '**build/coreboot.rom**' image with the size of 8mb.

# Add custom files to 'coreboot.rom' image

Add '**grub.cfg**' and '**background.jpg**' to coreboot.rom. See
'**Prerequisites**' above for the files needed.

Make sure '**cbfstool**' is built:

```sh
cd coreboot-4.15/util/cbfstool
make -j4
```

Add custom GRUB configuration file:

```sh
./cbfstool ../../build/coreboot.rom add -t raw -n etc/grub.cfg -f your/path/to/grub.cfg
```

Check so that '**etc/grub.cfg**' exists in coreboot.rom:

```sh
./cbfstool ../../build/coreboot.rom print
```

Add background image:

```sh
./cbfstool ../../build/coreboot.rom add -t raw -n background.jpg -f your/path/to/background.jpg
```

Check so that '**background.jpg**' exists in coreboot.rom:

```sh
./cbfstool ../../build/coreboot.rom print
```

# Add Descriptor and GbE File

Download libreboot utils:

```sh
wget https://mirrors.mit.edu/libreboot/stable/20160907/libreboot_r20160907_util.tar.xz
```

Extract the utils:

```sh
tar xvf libreboot_r20160907_util.tar.xz
```

Generate MAC address (MAC address is listed on the bottom of the X200 on
a white label on the right side):

```sh
cd libreboot_r20160907_util/ich9deblob/x86_64
./ich9gen --macaddress xx:xx:xx:xx:xx:xx
```

And lastly append it to coreboot.rom:

```sh
dd if=ich9fdgbe_8m.bin of=~/misc-src/coreboot-4.15/build/coreboot.rom bs=1 count=12k conv=notrunc
```

Done! Now it's time to flash the new '**coreboot.rom**' image!

# Flash coreboot image

See '**Connect programmer to flash chip**' for details on how
to connect programmer.

Flash coreboot using **Raspberry Pi** programmer:

```sh
sudo flashrom -p linux_spi:dev=/dev/spidev0.0,spispeed=32768 -w coreboot.rom -V
```

Flash coreboot using **ch341a** USB programmer:

```sh
sudo flashrom -p ch341a_spi -w coreboot.rom -V
```

# Change Wifi card

Remove the following parts to access the Wifi card:

```plaintext
Battery
Keyboard
Palmrest
```

See '**Connect programmer to flash chip**' for details on
where the Wifi card is located.

Now you can install a libre distro such as [Debian
GNU/Linux](https://www.debian.org/).

# Boot Debian GNU/Linux netinst iso from usb in GRUB2

Download Debian GNU/Linux netinst from [here](https://www.debian.org/).

Check name of USB device:

```sh
sudo fdisk -l
```

Create bootable USB drive:

```sh
sudo dd bs=4M if=/path/to/debian-img.iso of=/dev/YOUR_USB status=progress oflag=sync
```

Start the computer up and in the GRUB2 menu press '**c**' to enter
command line and from there type the following:

```sh
set root='usb0'
linux /install.amd/vmlinuz
initrd /install.amd/initrd.gz
boot
```

Install Debian GNU/Linux normally and put GRUB2 on master boot record at
the end of installation process. After reboot default option in GRUB2
menu on the flashchip will load GRUB2 on the AHCI HDD you installed on.

# Download compiled coreboot image

The resulting coreboot image from this tutorial can be downloaded here:

[`lenovo-thinkpad-x200_coreboot-4-15-grub-master-with-cpu-microcode.rom`](/static/blog/files/x200/lenovo-thinkpad-x200_coreboot-4-15-grub-master-with-cpu-microcode.rom)

Congratulations! We’re done.

# Recommended reading

[https://www.coreboot.org/](https://www.coreboot.org/)  
[https://www.flashrom.org/](https://www.flashrom.org/)  
[https://www.thinkwiki.org/wiki/Category:X200](https://www.thinkwiki.org/wiki/Category:X200)  
[https://download.lenovo.com/ibmdl/pub/pc/pccbbs/mobiles_pdf/43y6632_03.pdf](https://download.lenovo.com/ibmdl/pub/pc/pccbbs/mobiles_pdf/43y6632_03.pdf)  
[https://dejavu-fonts.github.io/](https://dejavu-fonts.github.io/)  
[https://dejavu-fonts.github.io/License.html](https://dejavu-fonts.github.io/License.html)
