# amdfix

**amdfix** is a tool born out of the need to address specific issues with AMD CPUs, particularly those found in Lenovo laptops using Ryzen processors. 🔧 This utility gathers a set of commands and configurations to disable *Turbo Boost* or tweak key parameters, improving system stability, especially on Linux platforms. 🛠️

## Introduction

The project started when my Lenovo Y530 (model 15ARH05 with a Ryzen 7 4800H and GTX 1650 Ti) began experiencing hardware issues. 💻 It all began with a capacitor failure near the Wi-Fi module, which led me to send the laptop in for repair under warranty. Lenovo quickly diagnosed that the motherboard needed replacement, and they also discovered that the RAM had been damaged due to the capacitor failure, downgrading my 32GB setup to 16GB temporarily. 🔄

After receiving the repaired laptop, I encountered frequent, random crashes on both Linux and Windows. Despite trying various solutions—swapping out RAM, using different SSDs, disconnecting the battery, and more—the issue persisted. 😔 Through extensive research, [I found a discussion indicating that some Ryzen 4800H processors, when paired with Lenovo motherboards, could experience instability due to issues with *Turbo Boost*](https://community.amd.com/t5/processors/laptop-ryzen-7-4800h-causes-random-crashes-and-occasional-bsods/td-p/614766). 🚀

By disabling the `/sys/devices/system/cpu/cpufreq/boost` parameter on Linux, I was finally able to stabilize the system. This discovery inspired the creation of **amdfix**, a tool designed to simplify the process of applying such fixes, making it easier for others who might be facing similar challenges. 🌟



## Features

- Disables Turbo Boost on AMD CPUs to improve stability. ⚙️
- Optimizes power management settings. 🔋 <b>(TODO)</b>
- Tools for diagnosing and resolving common AMD and Linux platform issues. 🛠️ <b>(TODO)</b>

## Requirements

- Linux (tested primarily on Ubuntu and Arch-based distributions). 🐧
- AMD CPU (recommended Ryzen 4000 series or newer). 🔍

## Installation

```bash
git clone https://github.com/root-hunter/amdfix.git
cd amdfix
chmod +x install.sh
sudo ./install.sh
```

## Usage
Activate CPU Turbo boost
```bash
amdfix boost active
```
Deactivate CPU Turbo boost
```bash
amdfix boost deactive
```
Check CPU Turbo boost status 
```bash
amdfix boost status
```