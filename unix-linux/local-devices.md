# Discovering devices on your Wi‑Fi — Step‑by‑step learning guide

**Goal:** Learn how to discover, identify, and manage devices connected to your Wi‑Fi network (e.g., finding your Raspberry Pi, spotting unknown devices, and taking security actions).

> ✅ Intended audience: beginners to intermediate users comfortable with a terminal (Linux/macOS/Windows/WSL).

---

## 1. Quick overview — what you'll learn

* How IP addressing and local subnets work (quick primer)
* How to find devices using your **router**, **nmap**, **arp**, **arp‑scan**, **ping sweeps**, and **mDNS/Bonjour**
* How to identify devices using **hostnames**, **MAC addresses / OUI vendor lookup**, and advertised services
* Practical commands for Linux, macOS, and Windows
* Troubleshooting tips and security best practices when you find an unknown device

---

## 2. Essential concepts (brief)

### What is an IP address and subnet?

* Devices on a local network have *private* IPv4 addresses like `192.168.1.x` or `10.0.0.x`.
* A subnet such as `192.168.1.0/24` groups addresses from `192.168.1.1` to `192.168.1.254`.
* Your router assigns addresses via **DHCP** and keeps a list called the **DHCP lease table**.

### Hostnames, mDNS, and `.local`

* Devices can broadcast names using **mDNS/Bonjour** enabling `raspberrypi.local` lookups on many networks.
* If `.local` lookups fail, mDNS may not be enabled on the client or network.

### MAC addresses and OUIs

* Every network interface has a **MAC address** (e.g. `b8:27:eb:aa:bb:cc`).
* The first 3 octets are the **OUI** and identify the vendor (helps guess device maker).

---

## 3. The fastest ways to find devices

### A. Check the router admin panel (recommended first step)

* Log into your router at its gateway address (common defaults: `192.168.0.1`, `192.168.1.1`, `10.0.0.1`).
* Look for **Connected devices**, **DHCP clients**, or **LAN status**.
* Pros: authoritative, shows hostnames, MACs, IPs. Cons: router UI differs across vendors.

### B. Use `nmap` network discovery (powerful and reliable)

* Install then run a ping scan to list active hosts:

Linux/Debian:

```bash
sudo apt update
sudo apt install nmap -y
sudo nmap -sn 192.168.1.0/24
```

macOS (Homebrew):

```bash
brew install nmap
sudo nmap -sn 192.168.1.0/24
```

Windows: use Nmap installer or WSL + the same command.

* `-sn` performs a ping/ARP discovery without port scanning.
* Look for results showing `Nmap scan report for <IP>` and `Nmap done`.

### C. ARP & ping sweep (works when nmap not available)

* Cause active hosts to populate your ARP cache by pinging range, then show ARP table.

Linux/macOS:

```bash
for ip in 192.168.1.{1..254}; do ping -c 1 -W 1 $ip >/dev/null & done; wait
arp -a
```

Windows PowerShell (ping sweep):

```powershell
1..254 | ForEach-Object { Test-Connection -Count 1 -Quiet ("192.168.1." + $_) }
arp -a
```

### D. `arp-scan` (fast and gives vendor names)

Linux:

```bash
sudo apt install arp-scan -y
sudo arp-scan --localnet
```

* `arp-scan` often lists vendor names next to MACs (useful to spot Raspberry Pi Foundation or other vendors).

### E. mDNS / Avahi / dns-sd to find advertised services

* macOS: `dns-sd -B _ssh._tcp` lists devices advertising SSH via mDNS.
* Linux (Avahi): `avahi-browse -a`.

### F. Mobile apps

* **Fing** (iOS/Android) quickly scans and gives friendly names and vendor guesses.

---

## 4. Commands cheat‑sheet (copy/paste friendly)

**Discover your local gateway / subnet**

```bash
# Linux/macOS
ip route | grep default
# or
ip -4 addr show
```

**nmap ping scan**

```bash
sudo nmap -sn 192.168.1.0/24
```

**arp-scan**

```bash
sudo arp-scan --localnet
```

**Populate ARP table then show**

```bash
for ip in 192.168.1.{1..254}; do ping -c 1 -W 1 $ip >/dev/null & done; wait
arp -a
```

**mDNS browse for SSH**

```bash
# macOS
dns-sd -B _ssh._tcp
# Linux with avahi
avahi-browse -a
```

**SSH into common headless device**

```bash
ssh pi@192.168.1.102
```

---

## 5. How to identify what a discovered device likely is

1. **Hostname** — `raspberrypi`, `nas`, `iphone` are obvious clues.
2. **MAC OUI vendor** — compare the MAC's first 3 bytes to vendor lists (e.g., Raspberry Pi OUIs include `b8:27:eb`, `dc:a6:32`, `e4:5f:01`, though these change with models).
3. **Open services** — `nmap -sS -p 22,80,443 <IP>` to see if SSH / HTTP / HTTPS are open (careful: scanning ports should be done on networks you own).
4. **mDNS/Bonjour services** — devices often advertise friendly names and services.
5. **Web UI** — some devices expose a web interface at their IP (try opening in a browser).

---

## 6. Example outputs and how to read them

**nmap** snippet (example):

```
Nmap scan report for 192.168.1.102
Host is up (0.00063s latency).
MAC Address: B8:27:EB:AA:BB:CC (Raspberry Pi Foundation)
```

* `Host is up` confirms the IP is active.
* `MAC Address` line gives vendor hint.

**arp -a** snippet (example):

```
? (192.168.1.102) at b8:27:eb:aa:bb:cc [ether] on wlan0
```

* Shows IP ↔ MAC mapping and interface where seen.

---

## 7. Troubleshooting when you can’t find the device

* Confirm Wi‑Fi credentials saved correctly on the device (recheck the `wpa_supplicant.conf` if headless Raspberry Pi).
* Ensure the device is powered and within Wi‑Fi range.
* Check router’s guest network — device might be on a different SSID.
* `.local` names failing: ensure mDNS/Avahi/Bonjour is running on client and your OS supports it.
* If device uses static IP in a different subnet, scan the full 10.0.x.x or other expected ranges.

---

## 8. If you find an unknown or unauthorized device

1. Immediately change your Wi‑Fi password (update devices afterward).
2. Consider enabling WPA2/WPA3, disabling WPS, and using a guest network for visitors.
3. Reboot the router to force reauthentication.
4. Optionally, enable MAC filtering or a firewall, but understand MACs can be spoofed.

---

## 9. Legal & ethical note

Only scan networks you own or have explicit permission to scan. Active scanning tools (nmap, arp‑scan) can be considered intrusive on networks you don’t control.

---

## 10. Next steps & resources

* `nmap` docs: [https://nmap.org](https://nmap.org)
* `arp-scan` docs: [https://github.com/royhills/arp-scan](https://github.com/royhills/arp-scan)
* Raspberry Pi headless setup docs: [https://www.raspberrypi.org/documentation/](https://www.raspberrypi.org/documentation/)
* Try a guided lab: set up a second smartphone as a test device and practice discovering it with `nmap` and `arp-scan`.

---

## 11. Want me to make a shorter quick reference card or a walkthrough for your OS?

Tell me which OS you're using (Linux/macOS/Windows) and I’ll create a one‑page cheat sheet or an exact step‑by‑step walkthrough with commands tailored to your environment.

*End of guide.*
