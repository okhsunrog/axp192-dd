# Page 1

![img-0.jpeg](img-0.jpeg)

# DATASHEET

V1.1 Mar 20 2015

# AXP192

Enhanced single Cell Li-Battery and Power System Management IC

# Page 2

# Revision History

|  Revision | Date | Description  |
| --- | --- | --- |
|  V1.0 | Jun.08,2010 | Initial Release Version  |
|  V1.1 | Mar.20,2015 | Change the format of the document  |

# Page 3

# Declaration 

X-Powers cannot assume responsibility for use of any circuitry other than circuitry entirely embodied in an X-Powers product. No circuit patent licenses, copyrights, or other intellectual property rights are implied. X-Powers reserves the right to make changes to the specifications and products at any time without notice.

# Page 4

# Catalogue 

Revision History ..... 2
Declaration ..... 3
Catalogue ..... 4

1. Summary ..... 7
2. Feature ..... 8
3. Typical Application ..... 9
4. Absolute Maximum Ratings ..... 10
5. Electrical Characteristics ..... 10
6. Typical Characteristics ..... 13
7. Pin Description ..... 17
8. Functional Block Diagram ..... 19
9. Control and Operating ..... 20
9.1. On/Off and Reset ..... 20
9.2. IPS ..... 21
9.3. Adaptive Charger ..... 23
9.4. Backup Battery ..... 26
9.5. Multi-Outputs ..... 27
9.6. Default Voltage/Timing Setting ..... 28
9.7. Signal Capture ..... 28
9.8. Multi-Function Pin Description ..... 29
9.9. Timer ..... 30
9.10. TWSI and IRQ ..... 30
9.11. Registers ..... 32
9.11.1. power supply control class ..... 32
9.11.2. GPIO control class ..... 33
9.11.3. IRQ control class ..... 33
9.11.4. ADC data class ..... 34
9.11.5. REG 00H: power supply status ..... 35
9.11.6. REG 01H: power supply work mode and charging status indicator ..... 35
9.11.7. REG 04H:USB OTG VBUS status indicator ..... 36
9.11.8. REG 06-09H:data buffer 0-3 ..... 36
9.11.9. REG 10H:EXTEN \& DC-DC2 output control ..... 36
9.11.10. REG 12H:DC-DC1/3 \& LDO2/3 output control ..... 36
9.11.11. REG 23H:DC-DC2 output voltage set ..... 37
9.11.12. REG 25H: DC-DC2 dynamic voltage parameter set ..... 37
9.11.13. REG 26H:DC-DC1 output voltage set ..... 37
9.11.14. REG 27H:DC-DC3 output voltage set ..... 38
9.11.15. REG 28H:LDO2/3 output voltage set ..... 38
9.11.16. REG 30H:VBUS-IPSOUT access management ..... 38
9.11.17. REG 31H:V ${ }_{\text {OFF }}$ power off voltage set ..... 39

# Page 5

9.11.18. REG 32H:power off、 battery check and CHGLED pin control ..... 39
9.11.19. REG 33H: charging control 1 ..... 40
9.11.20. REG 34H: charging control 2 ..... 40
9.11.21. REG 35H: Backup battery charge control ..... 40
9.11.22. REG 36H:PEK press key parameter set ..... 41
9.11.23. REG 37H:DC-DC work frequency set ..... 41
9.11.24. REG 38H:V ${ }_{\text {LTF-charge }}$ battery charging low temperature threshold set ..... 41
9.11.25. REG 39H:V ${ }_{\text {HTF-charge }}$ battery charging high temperature threshold set ..... 42
9.11.26. REG 3AH:APS low voltage level 1 ..... 42
9.11.27. REG 3BH:APS low voltage level 2 ..... 42
9.11.28. REG 3CH:V ${ }_{\text {LTF-discharge }}$ battery discharging low temperature threshold set ..... 42
9.11.29. REG 3DH:V ${ }_{\text {HTF-discharge }}$ battery discharging high temperature threshold set ..... 42
9.11.30. REG 80H:DC-DC work mode choose ..... 43
9.11.31. REG 82H:ADC enable 1 ..... 43
9.11.32. REG 83H:ADC enable 2 ..... 43
9.11.33. REG 84H: ADC sample rate set, TS pin control ..... 44
9.11.34. REG 85H: ADC input range ..... 44
9.11.35. REG 8AH: timer control ..... 44
9.11.36. REG 8BH:VBUS pin monitor SRP function control ..... 45
9.11.37. REG 8FH: over temperature power off function set ..... 45
9.11.38. REG 90H:GPIO0 function set ..... 45
9.11.39. REG 91H:GPIO in LDO mode, output voltage set ..... 46
9.11.40. REG 92H:GPIO1function set ..... 46
9.11.41. REG 93H:GPIO2 function set ..... 46
9.11.42. REG 94H:GPIO[2:0] signal status set and monitor ..... 47
9.11.43. REG 95H:GPIO[4:3] pin function set ..... 47
9.11.44. REG 96H: GPIO[4:3] signal status set and monitor ..... 47
9.11.45. REG 97H: when GPIO[2:0] is used as input, pull down set ..... 48
9.11.46. REG 98H:PWM1 output frequency set ..... 48
9.11.47. REG 99H:PWM1 duty ratio set 1 ..... 48
9.11.48. REG 9AH:PWM1 duty ratio set 2 ..... 48
9.11.49. REG 9BH:PWM2 output frequency set ..... 48
9.11.50. REG 9CH:PWM2 duty ratio set 1 ..... 49
9.11.51. REG 9DH:PWM2 duty ratio set 2 ..... 49
9.11.52. REG 9EH: N_RSTO pin function set ..... 49
9.11.53. REG 40H:IRQ enable 1 ..... 49
9.11.54. REG 41H:IRQ enable 2 ..... 50
9.11.55. REG 42H:IRQ enable 3 ..... 50
9.11.56. REG 43H:IRQ enable 4 ..... 51
9.11.57. REG 44H:IRQ status1 ..... 51
9.11.58. REG 45H:IRQ status2 ..... 51
9.11.59. REG 46H:IRQ status3 ..... 52
9.11.60. REG 47H:IRQ status4 ..... 52

# Page 6

# AXP192 

Enhanced single Cell Li-Battery and Power System Management IC
9.11.61. REG B8H: Coulomb counter control ..... 52
10. Package ..... 53

# Page 7

# 1. Summary 

The AXP192 provides an easy to use, fully integrated, ultra-flexible power solution for single cell Li-lon/Li-Polymer battery and multiple-power applications.

AXP192 contains an USB-Compatible charger, 3 Buck DC-DC converter, 4 low dropout linear regulator, voltage / current / temperature monitor and multi channel 12-Bit ADC. To ensure power system work stably, AXP192 also contains self-protection circuits such as OVP, UVP, OTP and OCP .

The "Intelligent Power Select"(IPS) ${ }^{\text {TM }}$ circuit of AXP192 distributes power safely and transparently between the USB, external AC adapter, Li-Battery and the application system. It also allows the application system work without battery or discharged battery.

AXP192 has three input source, including external adapter, USB VBUS input and battery. It also supports use of the rechargeable backup battery.

AXP192 provides TWSI(Two Wire Serial Interface) to communicate with the application processor. Through the interface, processor can enable or disable the outputs, and set the output voltage, as well as get the power status and "fuel gauge" data, High-precision ADC makes it convenient for consumer real-time control and know the system power dissipation, which brings them wonderful experience of device electricity usage that never had before.

AXP192 is available in a 48-pin $6 \mathrm{~mm} \times 6 \mathrm{~mm}$ QFN package.

## Applications

- Handhold mobile devices Smart cell phone, PMP/MP4, digital camera, handhold navigation devices GPS, PDA, digital broadcast TV receiver
- MID(Mobile internet device)
- Digital photo Frame, portable DVD player, UMPC, and UMPC-like, Learning machine
- Application Processor systems
- Other battery and multi-power applications


## Pin Description

![img-1.jpeg](img-1.jpeg)

# Page 8

# 2. Feature 

## - Battery Management

- Operation Voltage:
$2.9 \mathrm{~V} \sim 6.3 \mathrm{~V}$ (AMR: $-0.3 \mathrm{~V} \sim 11 \mathrm{~V}$ )
- Configurable Intelligent Power Select system
- Current and voltage limit of adaptive USB or AC adapter input
- The resistance of internal ideal diode lower than $100 \mathrm{~m} \Omega$


## - Full-integrated Charger

- 1.4A charge current with built-in MOSFET
- Battery temperature monitor
- USB-Compatible charger
- High precision as $0.5 \%$
- support 4.1V/4.15V/4.2V/4.36V battery
- Charging process control automatically
- LED driver to indication the charging status
- Auto adjust the charging current according to the system load


## - Backup Battery

- Provide power to RTC module by using the backup battery
- Integrated an backup battery charger


## - 3 Synchronous Step-Down Converters

- DC-DC1: can be adjusted between $0.7 \mathrm{~V} \sim 3.5 \mathrm{~V}$ 25mV/step, 1.2A drive capability
- DC-DC2: can be adjusted between $0.7 \mathrm{~V} \sim 2.275 \mathrm{~V}$ 25mV/step, 1.6A drive capability, support VRC
- DC-DC3: can be adjusted between $0.7 \mathrm{~V} \sim 3.5 \mathrm{~V}$ 25mV/step, 0.7A drive capability


## - 4 LDO's

- LDO1:30mA, always on
- LDO2:low noise LDO, can be adjusted between $1.8 \mathrm{~V} \sim 3.3 \mathrm{~V}, 100 \mathrm{mV} /$ step, 200 mA drive capability
- LDO3:low noise LDO, can be adjusted between $1.8 \mathrm{~V} \sim 3.3 \mathrm{~V}, 100 \mathrm{mV} /$ step, 200 mA drive capability
- LDOIO0: low noise LDO, can be adjusted between $1.8 \mathrm{~V} \sim 3.3 \mathrm{~V}, 100 \mathrm{mV} /$ step, 50 mA drive capability

NOTE: VRC, Voltage Ramp Control

## - Signal Capture

- built-in 16 channel 12 Bit ADC
- 4 external input channels
- Built-in high precision coulomb counter and fuel-gauge system
- Wealthily power information, such as the real-time power dissipation ( mA or mW ), remaining battery status(\% or mAh), and remaining battery or charging time
- Low power warning and protection
- Provide temperature information of chip


## - Host Interface

- Host can exchange data with processor by TWSI
- Flexibility to configure the interrupt management
- Multi-function GPIO can be set to IO,PWM and other function
- Built-in timer
- Four registers can be used to save the data when system shutdown


## - System Management

- Soft reset or hardware reset
- Support soft shutdown or hardware shutdown, and external wakeup
- Monitoring output voltage, self-diagnostic function
- PWROK is used for system reset
- External power source detect (insert/remove/lack of driving capacity)
- Soft start
- Over voltage protection /under voltage protection (OVP/UVP)
- Over current protection (OCP)
- Over temperature protection (OTP)
- Support OTG VBUS power state setting/monitoring
- Fully Integration
- High-accuracy internal Reference Voltage ( $0.5 \%$ )
- Built-in MOSFET

# Page 9

# 3. Typical Application 

![img-2.jpeg](img-2.jpeg)

# Page 10

# 4. Absolute Maximum Ratings

|  Symbol | Description | Value | Units  |
| --- | --- | --- | --- |
|  ACIN | Input Voltage | -0.3 to 11 | V  |
|  VBUS | Input Voltage | -0.3 to 11 | V  |
|  $T_{i}$ | Operating Temperature Range | -40 to 130 | ${ }^{\circ} \mathrm{C}$  |
|  Ts | Storage Temperature Range | -40 to 150 | ${ }^{\circ} \mathrm{C}$  |
|  $T_{\text {LEAD }}$ | Maximum Soldering Temperature (at leads, 10sec) | 300 | ${ }^{\circ} \mathrm{C}$  |
|  $\mathrm{V}_{\text {ESD }}$ | Maximum ESD stress voltage, Human Mode | $>4000$ | V  |
|  $P_{D}$ | Internal Power Dissipation | 2100 | mW  |

## 5. Electrical Characteristics

$\mathrm{V}*{\mathrm{IN}}=5 \mathrm{~V}, \quad$ BAT $=3.8 \mathrm{~V}, \quad \mathrm{~T}*{\mathrm{A}}=25^{\circ} \mathrm{C}$

|  SYMBOL | DESCRIPTION | CONDITIONS | MIN | TYP | MAX | UNITS  |
| --- | --- | --- | --- | --- | --- | --- |
|  ACIN |  |  |  |  |  |   |
|  $\mathrm{V}_{\text {IN }}$ | ACIN Input Voltage |  | 3.8 |  | 6.3 | V  |
|  $\mathrm{I}_{\text {OUT }}$ | $\mathrm{V}_{\text {OUT }}$ Current Available Before Loading BAT | 500mV Voltage Drop |  | 2000 |  | mA  |
|  $\mathrm{V}_{\text {UVLO }}$ | ACIN Under Voltage Lockout |  |  | 3.8 |  | V  |
|  $\mathrm{V}_{\text {OUT }}$ | IPS Output Voltage |  | 2.9 |  | 5.0 | V  |
|  $\mathrm{R}_{\text {ACIN }}$ | Internal Ideal Diode On Resistance | PIN to PIN, ACIN to IPSOUT |  |  | 200 | $\mathrm{m} \Omega$  |
|  VBUS |  |  |  |  |  |   |
|  $\mathrm{V}_{\text {IN }}$ | VBUS Input Voltage |  | 3.8 |  | 6.3 | V  |
|  $\mathrm{I}_{\text {OUT }}$ | $\mathrm{V}_{\text {OUT }}$ Current Available Before Loading BAT | 400mV Voltage Drop |  | 500 | 900 | mA  |
|  $\mathrm{V}_{\text {UVLO }}$ | VBUS Under Voltage Lockout |  |  | 3.8 |  | V  |
|  $\mathrm{V}_{\text {OUT }}$ | IPS Output Voltage |  | 2.9 |  | 5.0 | V  |
|  $\mathrm{R}_{\text {VBUS }}$ | Internal Ideal Diode On Resistance | PIN to PIN, VBUS to IPSOUT |  |  | 300 | $\mathrm{m} \Omega$  |
|  Battery Charger |  |  |  |  |  |   |
|  $\mathrm{V}_{\text {TRGT }}$ | BAT Charge Target Voltage |  | $-0.5 \%$ | 4.2 | $+0.5 \%$ | V  |
|  $\mathrm{I}_{\text {CHRG }}$ | Charge Current |  |  | 780 | 1320 | mA  |
|  $\mathrm{I}_{\text {TRKL }}$ | Trickle Charge Current |  |  | $10 \%$ |  | $\mathrm{I}_{\text {CHRG }} \mathrm{mA}$  |
|  $\mathrm{V}_{\text {TRKL }}$ | Trickle Charge Threshold Voltage |  |  | 3.0 |  | V  |
|  $\Delta \mathrm{V}_{\text {RECHG }}$ | Recharge Battery Threshold Voltage | Threshold Voltage Relative to $\mathrm{V}_{\text {TARGET }}$ |  | $-100$ |  | mV  |

# Page 11

| $\mathrm{T}_{\text {TIMER1 }}$ | Charger Safety Timer Termination Time | Trickle Mode |  | 40 |  | Min |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $\mathrm{T}_{\text {TIMER2 }}$ | Charger Safety Timer Termination Time | CC Mode |  | 480 |  | Min |
| $\mathrm{I}_{\text {END }}$ | End of Charge Indication Current Ratio | CV Mode |  | $10 \%$ | $15 \%$ | $\mathrm{I}_{\text {CHRG }} \mathrm{mA}$ |
| Backup Battery |  |  |  |  |  |  |
| $\mathrm{V}_{\text {TRGT }}$ | Backup Battery Charge Target Voltage |  | 2.5 | 3.0 | 3.1 | V |
| $\mathrm{I}_{\text {CHRG }}$ | Backup Battery Charge Current |  | 50 | 200 | 400 | uA |
| $\mathrm{I}_{\text {Backup }}$ | Current when use Backup Battery |  |  | 10 | 15 | uA |
| NTC |  |  |  |  |  |  |
| $\mathrm{V}_{\mathrm{TL}}$ | Cold Temperature Fault Threshold Voltage | Charge <br> Discharge | 0 | $\begin{aligned} & 2.112 \\ & 3.226 \end{aligned}$ | 3.264 | V |
| $\mathrm{V}_{\mathrm{TH}}$ | Hot Temperature Fault Threshold Voltage | Charge <br> Discharge | 0 | $\begin{aligned} & 0.397 \\ & 0.282 \end{aligned}$ | 3.264 | V |
| $\mathrm{V}_{\mathrm{TE}}$ | NTC Disable Threshold Voltage | Falling Threshold Hysteresis |  | 0.2 |  | V |
| Ideal Diode |  |  |  |  |  |  |
| $R_{\text {ds(on) }}$ | Internal Ideal Diode On Resistance(BAT to IPSOUT) |  |  |  | 100 | $\mathrm{m} \Omega$ |


| SYMBOL | DESCRIPTION | CONDITIONS | MIN | TYP | MAX | UNITS |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| Off Mode Current |  |  |  |  |  |  |
| $\mathrm{I}_{\text {BATOFF }}$ | OFF Mode Current | $\mathrm{BAT}=3.8 \mathrm{~V}$ |  | 27 |  | $\mu \mathrm{A}$ |
| $\mathrm{I}_{\text {SUSPEND }}$ | USB VBUS suspend Mode current | $\begin{aligned} & \mathrm{BAT}=3.8 \mathrm{~V}, \\ & \mathrm{VBUS}=5 \mathrm{~V}, \\ & \mathrm{~N} \_\mathrm{VBUSEN}=1 \end{aligned}$ |  | 86 |  | $\mu \mathrm{A}$ |
| Logic |  |  |  |  |  |  |
| $\mathrm{V}_{\mathrm{IL}}$ | Logic Low Input Voltage |  |  | 0.3 |  | V |
| $\mathrm{V}_{\mathrm{IN}}$ | Logic High Input Voltage |  |  | 2 |  | V |
| TWSI |  |  |  |  |  |  |
| $\mathrm{V}_{\mathrm{CC}}$ | Input Supply Voltage |  |  | 3.3 |  | V |
| ADDRESS | TWSI Address |  |  | 0x68 |  |  |
| $f_{\text {SCX }}$ | Clock Operating Frequency |  |  | 400 | 1200 | kHZ |
| $t_{f}$ | Clock Data Fall Time | 2.2Kohm Pull High |  | 60 |  | ns |
| $t_{r}$ | Clock Data Rise Time | 2.2Kohm Pull High |  | 100 |  | ns |
| DCDC |  |  |  |  |  |  |
| $\mathrm{f}_{\text {OSC }}$ | Oscillator Frequency | Default |  | 1.5 |  | MHz |
| DCDC1 |  |  |  |  |  |  |

# Page 12

# AXP192

Enhanced single Cell Li-Battery and Power System Management IC

|  $\mathrm{I}_{\text {VIN1 }}$ | Input Current | PFM Mode
$\mathrm{I}_{\text {DC1OUT }}=0$ |  | 26 |  | $\mu \mathrm{A}$  |
| --- | --- | --- | --- | --- | --- | --- |
|  $\mathrm{I}_{\text {UM1 }}$ | PMOS Switch Current Limit | PWM Mode |  | 1600 |  | mA  |
|  $\mathrm{I}_{\text {DC1OUT }}$ | Available Output Current | PWM Mode |  | 1200 |  | mA  |
|  $\mathrm{V}_{\text {DC1OUT }}$ | Output Voltage | Default | 0.7 | 3.3 | 3.5 | V  |
|  DCDC2 |  |  |  |  |  |   |
|  $\mathrm{I}_{\text {VIN2 }}$ | Input Current | PFM Mode
$\mathrm{I}_{\text {DC2OUT }}=0$ |  | 20 |  | $\mu \mathrm{A}$  |
|  $\mathrm{I}_{\text {UM2 }}$ | PMOS Switch Current Limit | PWM Mode |  | 2300 |  | mA  |
|  $\mathrm{I}_{\text {DC2OUT }}$ | Available Output Current | PWM Mode |  | 1600 |  | mA  |
|  $\mathrm{V}_{\text {DC2OUT }}$ | Output Voltage Range |  | 0.7 | 1.25 | 2.275 | V  |
|  DCDC3 |  |  |  |  |  |   |
|  $\mathrm{I}_{\text {VIN3 }}$ | Input Current | PFM Mode
$\mathrm{I}_{\text {DC3OUT }}=0$ |  | 20 |  | uA  |
|  $\mathrm{I}_{\text {UM3 }}$ | PMOS Switch Current Limit | PWM Mode |  | 1000 |  | mA  |
|  $\mathrm{I}_{\text {DC3OUT }}$ | Available Output Current | PWM Mode |  | 700 |  | mA  |
|  $\mathrm{V}_{\text {DC3OUT }}$ | Output Voltage Range |  | 0.7 | 2.5 | 3.5 | V  |

|  SYMBOL | DESCRIPTION | CONDITIONS | MIN | TYP | MAX | UNITS  |
| --- | --- | --- | --- | --- | --- | --- |
|  LDO1 |  |  |  |  |  |   |
|  $\mathrm{V}_{\text {LDO1 }}$ | Output Voltage | $\mathrm{I}_{\mathrm{LDO} 1}=1 \mathrm{~mA}$ | $-1 \%$ | $\begin{aligned} & 1.25 \ & 1.8 \ & 2.5 \ & 3.3 \end{aligned}$ | $1 \%$ | V  |
|  $\mathrm{I}_{\text {LDO1 }}$ | Output Current |  |  | 30 |  | mA  |
|  LDO2 |  |  |  |  |  |   |
|  $\mathrm{V}_{\text {LDO2 }}$ | Output Voltage | $\mathrm{I}_{\mathrm{LDO} 2}=1 \mathrm{~mA}$ | $-1 \%$ | 3 | $1 \%$ | V  |
|  $\mathrm{I}_{\text {LDO2 }}$ | Output Current |  |  | 200 |  | mA  |
|  $\mathrm{I}_{\mathrm{Q}}$ | Quiescent Current |  |  | 100 |  | $\mu \mathrm{A}$  |
|  PSRR | Power Supply Rejection Ratio | $\mathrm{I}*{\text {LDO2 }}=60 \mathrm{~mA}, 1 \mathrm{KHz}$ |  | TBD |  | dB  |
|  $\mathrm{e}_{\mathrm{N}}$ | Output Noise,20-80KHz | Vo=3V, Io=150mA |  | 28 |  | $\mu \mathrm{V}_{\text {RMS }}$  |
|  LDO3 |  |  |  |  |  |   |
|  $\mathrm{V}_{\text {LDO3 }}$ | Output Voltage | $\mathrm{I}_{\mathrm{LDO} 3}=1 \mathrm{~mA}$ | $-1 \%$ | 3.3 | $1 \%$ | V  |
|  $\mathrm{I}_{\text {LDO3 }}$ | Output Current |  |  | 200 |  | mA  |
|  $\mathrm{I}_{\mathrm{Q}}$ | Quiescent Current |  |  | 100 |  | $\mu \mathrm{A}$  |
|  PSRR | Power Supply Rejection Ratio | $\mathrm{I}*{\text {LDO3 }}=10 \mathrm{~mA}, 1 \mathrm{KHz}$ |  | TBD |  | dB  |
|  $\mathrm{e}_{\mathrm{N}}$ | Output Noise,20-80KHz | Vo=1.8V, Io=150mA |  | 18 |  | $\mu \mathrm{V}_{\text {RMS }}$  |
|  $\mathrm{LDO}_{\text {IOO }}$ |  |  |  |  |  |   |
|  $\mathrm{V}_{\text {LDOIOO }}$ | Output Voltage | $\mathrm{I}_{\text {LDOIOO }}=1 \mathrm{~mA}$ | $-1 \%$ | 3.3 | $1 \%$ | V  |
|  $\mathrm{I}_{\text {LDOIOO }}$ | Output Current |  |  | 50 |  | mA  |
|  $\mathrm{I}_{\mathrm{Q}}$ | Quiescent Current |  |  | 90 |  | $\mu \mathrm{A}$  |

# Page 13

# AXP192 

Enhanced single Cell Li-Battery and Power System Management IC

| PSRR | Power Supply Rejection Ratio | $\mathrm{I}_{100 \mathrm{~K} \mathrm{~K} \mathrm{~S}}=10 \mathrm{~mA}, 1 \mathrm{KHz}$ |  | TBD |  | dB |
| :--: | :--: | :--: | :--: | :--: | :--: | :--: |
| $e_{N}$ | Output Noise,20-80KHz | Vo=1.8V, lo=30mA |  | 18 |  | $\mu \mathrm{V}_{\text {RMS }}$ |

## 6. Typical Characteristics

![img-3.jpeg](img-3.jpeg)

# Page 14

![img-4.jpeg](img-4.jpeg)

![img-5.jpeg](img-5.jpeg)

![img-6.jpeg](img-6.jpeg)

# AXP192

## Enhanced single Cell Li-Battery and Power System Management IC

### DC-DC Ripple

|  Pin | 8.8 | Vertical | Mods/Ach | Tng | Display | Careers | Minimare | Mark | Meln | MySospn | Amacse | Utilities | Help | Tx  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |   |   |   |   |   |   |   |   |   |   |   |   |   |   |

### VREF vs Temperature

# Page 15

![img-7.jpeg](img-7.jpeg)

# AXP192

Enhanced single Cell Li-Battery and Power System Management IC

![img-8.jpeg](img-8.jpeg)

## Vref vs Temp

## VTRGT vs Temperature

# Page 16

![img-9.jpeg](img-9.jpeg)

# AXP192

Enhanced single Cell Li-Battery and Power System Management IC

![img-10.jpeg](img-10.jpeg)

## w

- **W** 20
- **w** 10
- **w** 20
- **w** 30
- **w** 40
- **w** 50
- **w** 60
- **w** 70
- **w** 80

## c

- **Off Mode Current vs Vbatt**
- **Vbat Off Mode Current**

## uA

- **29**
- **28**
- **27**
- **26**
- **25**
- **24**
- **3.3**
- **3.4**
- **3.5**
- **3.6**
- **3.7**
- **3.8**
- **3.9**
- **4**
- **4.1**
- **4.2**

![img-11.jpeg](img-11.jpeg)

# Page 17

# 7. Pin Description 

| Num | Name | Type | Condition | Function Description |
| :--: | :--: | :--: | :--: | :--: |
| 1 | SDA | IO |  | Data pin for serial interface, normally it connect a 2.2 K resistor to $3.3 \mathrm{~V} \mathrm{I} / \mathrm{O}$ power |
| 2 | SCK | I |  | It is the Clock pin for serial interface, normally it connect a 2.2 K resistor to $3.3 \mathrm{~V} \mathrm{I} / \mathrm{O}$ power |
| 3 | N_RSTO | IO | REG9EH[7] | LDO1 Reset output |
|  |  |  |  | GPIO[5] |
| 4 | N_OE | I |  | Power output on/off switch GND: on: IPSOUT: off |
| 5 | PWROK/ <br> N_LBO | 0 | SYSEN=LDO1 | Power good indication |
|  |  |  |  | Low power detect output |
| 6 | N_VBUSEN | I |  | VBUS to IPSOUT Selection <br> GND: IPSOUT select VBUS <br> High: IPSOUT do not select VBUS |
| 7 | VIN2 | PI |  | DCDC2 input source |
| 8 | LX2 | IO |  | Inductor Pin for DCDC2 |
| 9 | PGND2 | G |  | NMOS Ground for DCDC2 |
| 10 | DCDC2 | I |  | DC-DC2 feedback pin |
| 11 | LDO3 | 0 |  | Output Pin of LDO3 |
| 12 | LDO2 | 0 |  | Output Pin of LDO2 |
| 13 | LDOIN | PI |  | Input to LDO2 and LDO3 |
| 14 | VIN3 | PI |  | DCDC3 input source |
| 15 | LX3 | IO |  | Inductor Pin for DCDC3 |
| 16 | PGND3 | G |  | NMOS GND for DCDC3 |
| 17 | DCDC3 | I |  | Feed back to DCDC3 |
| 18 | GPIO1 | $\begin{aligned} & \text { IO } \\ & \text { I } \end{aligned}$ | $\begin{aligned} & \text { REG } \\ & 93 \mathrm{H}[2: 0] \end{aligned}$ | GPIO 2 |
|  |  |  |  | PWM 2 |
|  |  |  |  | ADC Input |
| 19 | GPIO0 | $\begin{aligned} & \text { IO } \\ & \text { I } \end{aligned}$ | $\begin{aligned} & \text { REG } \\ & 90 \mathrm{H}[2: 0] \end{aligned}$ | GPIO 0 |
|  |  |  |  | Low noise LDO |
|  |  |  |  | ADC Input |
| 20 | GPIO2 | IO | $\begin{aligned} & \text { REG } \\ & 92 \mathrm{H}[2: 0] \end{aligned}$ | GPIO 1 |
|  |  |  |  | PWM 1 |
|  |  |  |  | ADC Input |
| 21 | APS | PI |  | Internal Power Input |
| 22 | AGND | G |  | Analog Ground |

# Page 18

EXP192

Enhanced single Cell Li-Battery and Power System Management IC

|  23 | PWRON | I |  | Power On-Off key input, Internal 100k pull high to APS  |
| --- | --- | --- | --- | --- |
|  24 | BIAS | IO |  | External 200Kohm 1\% resistor  |
|  25 | VREF | 0 |  | Internal reference voltage  |
|  26 | VINT | PO |  | Internal logic power, 2.5 V  |
|  27 | PWREN | IO |  | It is the Low-voltage Power domain enable signal  |
|  28 | LDO1 | 0 |  | LDO1 output, for Host RTC block  |
|  29 | SYSEN | IO |  | It is the High-voltage Power domain enable signal  |
|  30 | BACKUP | IO |  | Backup battery pin  |
|  31 | VBUS | PI |  | USB VBUS input  |
|  32,33 | ACIN | PI |  | Adapter input  |
|  34,35 | BAT | IO |  | Main Battery  |
|  36 | CHGLED | 0 |  | charger status indication  |
|  37 | TS | I |  | Battery Temperature sensor input or an external ADC input  |
|  38,39 | IPSOUT | PO |  | System power source  |
|  40 | EXTEN | 0 |  | External power module Enable  |
|  41 | GPIO3 | I | REG95H[7] | GPIO3  |
|  42 | GPIO4 | I |  | GPIO4  |
|  43 | NC | 0 |  | NC  |
|  44 | VIN1 | PI |  | DCDC1 input source  |
|  45 | LX1 | IO |  | Inductor Pin for DCDC1  |
|  46 | PGND1 | G |  | NMOS Ground for DCDC1  |
|  47 | DCDC1 | I |  | DCDC1 feedback pin  |
|  48 | IRQ/WAKEUP | IO |  | IRQ output or wakeup  |
|  49 | EP | G |  | Exposed Pad, need to connect to system ground  |

# Page 19

# 8. Functional Block Diagram 

![img-12.jpeg](img-12.jpeg)

# Page 20

# 9. Control and Operating 

When AXP192 is working, the TWSI interface, SCK/SDA pin is pulled up to the system IO power, the Host can adjust and monitor the status of AXP192 through this interface.
Note: "Host" refers to the processor of the application system
Note: The following "external power" contains the input of ACIN and VBUS

### 9.1. On/Off and Reset

## PEK

A key can be connected between the pin PWRON and GND, which act as an independent switch key, called Power Enable Key (PEK). AXP192 can automatically identify that either the key-press is "long press" or "short press" and make the appropriate response.

## Power on trigger Source

1.ACIN, VBUS and the battery access.
2.N_OE changed from high to low.
3.PEK.

## Power On

When N_OE from high to low and the main power (ACIN or VBUS> 3.8V, the battery voltage is higher than the shutdown voltage) exist, AXP192 will automatically power on (whether automatically start up or not when the external power is supplied can be set according to external demand).

When N_OE is low and AXP192 is turned off, the PEK will result in the power on actions.

AXP192 can be started up by PEK (the time of pushing the key must be more than "ONLEVEL") . In practice, the timer (Alarm) timeout signal of the system can be connected to PWRON-and be parallel with PEK, low level signals is the equivalent of PEK press, also can cause AXP192 to be started up.

DC-DC and LDO will be soft started in order.

## Power Off

When the time of PEK "long-press" longer than IRQLEVEL, through the interrupt service program in PEK, Host can write "1" to the "register REG32H [7]" to inform AXP192 to shutdown, it will turn off all the output except LDO1.

In the following cases, AXP192 will automatically shut down:
1, the input voltage is too low, low power protection;
2, the output voltage too low, overload protection;
3, the input voltage is too high, over-voltage protection (details in the "power-path management" section);

# Page 21

4, N_OE changed from low to high, but AXP192 is not shutdown during the pre-define time;
5 , when PEK is longer than OFFLEVEL (default 6S) ,the system automatically disable all output except LDO1;

The mechanism of AXP192 automatic protection, can avoid un-recovery damage to the device when application exception occurs to protect the entire system.

# PWROK 

The PWROK of AXP192 can be used as the reset signal of the application system. During startup time, PWROK is low, after the output voltage of all channels stability, PWROK will be pulled up in order to assert a power-on reset to the system.

During the application system work as normal, AXP192 monitors the output voltage and load all the time, if in the overload or under voltage situation, PWROK immediately changes to be low, resets application system and then power off to protect the system.

### 9.2. IPS

AXP192's power input can come from lithium battery BAT, USB VBUS input, external power supply ACIN (such as AC adapter), according to the status of the external power and lithium battery, IPS selects the appropriate source.
o only lithium battery exist and no external power input, use lithium battery power to supply;
o access to an external power source (VBUS or ACIN), use external power to supply in priority;
o the battery is connected, when external power supply removed, immediately switch to lithium battery power supply;
o the VBUS and ACIN both exist, use the ACIN power in priority and charge on the lithium battery;
o If ACIN capability is not enough to system load and charger, it will open the VBUS path to achieve ACIN / VBUS work together;
o If the drive capacity is still insufficient, it will reduce the charge current until 0 , and then switch on the battery path to work together.

See the following diagram:

# Page 22

![img-13.jpeg](img-13.jpeg)

Enhanced single Cell Li-Battery and Power System Management IC

![img-14.jpeg](img-14.jpeg)

As shown, when ACIN load capacity is insufficient, IPSOUT voltage drops, BAT will be in change if status is from being charged to discharging, provides current for system load with ACIN together.

Through TWSI, Host can access AXP192 internal register to set the parameters of IPS and read the status.

### Voltage / Current limit mode and direct mode

In order to avoid influencing the USB communications, VBUS default works in "VBUS voltage-limiting mode." In this mode, AXP192 will hold VBUS voltage above at a reference voltage VHOLD to meet the USB specification. VHOLD is 4.4V at default, can be adjusted at register Reg30H [5:3].

If the system has the demands to limit the current which is drawn from the USB VBUS, a current limit mode is provided (see Register REG30H [1]), the optional value is 500mA/100mA (register Reg30H [0]).

If the system only uses the USB-powered and does not care USB communication, or use the USB-port power adapter, you can set AXP192 to "VBUS direct mode" through modify the register REG30H [6], then AXP192 will give priority to meet electricity demand for applications. If the USB Host drive capacities is too weak or Power Consumption is too strong, VBUS power system voltage will be less than VHOLD, AXP192 will assert IRQ, tell the Host that VBUS power supply capacity is weak and the USB communication may be invalid, the follow-up actions can be decided by the Host software.

### The reaction when the AXP192 is inserted by external power

AXP192 can automatically detect whether the external power inserted or not. After the external power is valid, it will automatically determine whether the external power supply is available to use or not, and the results will be set in the corresponding register, at the same time assert IRQ.

# Page 23

Register status bits of the external power and the meaning in the following table:

|  REG00H[7] | Indication the existence of an external adapter power ACIN  |
| --- | --- |
|  REG00H[6] | Indication the available to use or not of the external power adapter ACIN  |
|  REG00H[5] | Indication the existence of an external power VBUS  |
|  REG00H[4] | Indication the available to use or not of an external power VBUS  |
|  REG00H[3] | When VBUS insert, whether VBUS voltage is above VHOLD or not  |
|  REG00H[1] | Whether external power ACIN / VBUS is in short the PCB or not  |
|  REG00H[0] | Whether the startup is triggered by the ACIN / VBUS or not  |

When Host receives IRQ7(mean VBUS is weak),together with the status REG00H[3],it will determine whether VBUS low voltage status is because of system load or VBUS is lower than $\mathrm{V}_{\text {HOLD }}$ at the time insert. That make the host decide the system work on voltage limit mode or change to direct mode.

Use VBUS as power supply or not AXP192 use VBUS as power supply or not, decided by the status of N_VBUSEN and REG30H[7]:

|  N_VBUSEN | REG30H[7] | Power Supply | Description  |
| --- | --- | --- | --- |
|  Low | 0 | VBUS | When VBUS is available and ACIN is unavailable, use
VBUS as power supply  |
|  Low | 1 | VBUS | When VBUS is available, can use VBUS as power
supply  |
|  High | 1 | VBUS | Not use VBUS  |
|  High | 0 | ACIN/BAT |   |

# Low power warning and low power protection (automatically power off)

AXP192 has two stage of low voltage warning and automatically power off ( $\mathrm{V}_{\text {OFF }}$ ) which is compared with APS. Once APS is lower than $\mathrm{V}_{\text {WARNING }}$, AXP192 sends out IRQ30. If APS is lower than $\mathrm{V}_{\text {OFF }}$, AXP192 gets into power off mode, disable all output except LDO1. $\mathrm{V}_{\text {WARNING }}$ can be set into LEVEL1/LEVEL2, when APS is lower than LEVEL2, AXP192 sends out IRQ30, after APS is higher than LEVEL1, clear this IRQ automatically.

The default value $\mathrm{V}_{\text {WARNING }}$ and $\mathrm{V}_{\text {OFF }}$ is set in REG3AH、REG3BH and REG31H Bit[2:0].

## Over-voltage protection

When external power supply voltage higher than 6.2 V , AXP192 sends out IRQ1 or IRQ4, which means external power supply is over-voltage. If higher than 7 V , AXP192 is powered off automatically.

### 9.3. Adaptive Charger

AXP192 has integrated a constant current/voltage charger, which can automatically control charge period, internal safety clock can stop charging without CPU. This charger can adjust charge current based on the power dissipation of the system, fuel gauge, small current charge and active mode. Internal temperature detect circuit

# Page 24

![img-15.jpeg](img-15.jpeg)

# AXP192

*Enhanced single Cell Li-Battery and Power System Management IC*

can decrease charge current when in over/under temperature.

## Startup adaptive charge

The charge function is default enable(register disable, see "REG33H"). After insert external power source, AXP192 judges whether external power source can be used for charge or not, when it's available, and charge function is enabled, AXP192 gets into charge mode automatically, and sends IRQ to Host, which indicates the charge process start. Meanwhile, CHGLED was set to low to drive external light-emitting diode indicate AXP192 is in charge mode.

## Charge voltage current diagram

![img-16.jpeg](img-16.jpeg)

### Two indicate voltage

VTRGT, charge target voltage. VTRGT can be set by register, 4.2V by default (see REG33H[6:5]). Meanwhile, when external power source is lower than 4.2V, AXP192 will adjust VTRGT by itself.

VRCH, automatic recharge voltage. VRCH=VRGT -0.1V.

### Charge current

Charge current can be set by REG33H[3:0], whose default value is 450mA or 780mA.

### Charge flow

If battery voltage is lower than 3.0V, charger goes into pre-charge mode, charge current is 1/10 of set value. If after 40 minutes (which can be set by REG34H), battery voltage can not reach 3.0V, the charger goes into active mode. See detail in "battery active mode".

Once battery is higher than 3.0V, charger goes into constant current mode. If charge current is lower than 65% of the set value, the system sends out IRQ17, which informs "External power source is weak, charge current

# Page 25

doesn't reach the target value, so the charge time will be longer, if you want to reduce the charge time, you should insert a more powerful source or disable the system load".

When the battery reaches $\mathrm{V}_{\text {TRGT }}$, the charger goes into constant voltage mode from constant current mode, charge current decrease.

In constant voltage mode, When charge current is lower than $10 \%$ or $15 \%$ of the set value(which can be set by REG33H), charge period is over, AXP192 sends out IRQ13, CHGLED indicate stop status. When battery voltage is lower than $\mathrm{V}_{\mathrm{RCH}}$ again, AXP192 can recharge automatically, and sends out IRQ12.

In non-pre-charge mode, if after 480 minutes(which can be set by REG34H), charge period is not over, the charger goes into battery active mode.

# Battery active mode 

In battery active mode (timing counter timeout), the charger sends out IRQ10, which indicate battery may be damaged.

In battery active mode, Charger use small current to charge battery all the time, if the battery voltage reaches $\mathrm{V}_{\mathrm{RCH}}$, the charger exits active mode, and sends out IRQ11.

AXP192 uses REG01H to indicate charger is in battery active mode or not.

## CHGLED

CHGLED pin indicate charge mode and warning mode, it has four statuss: charge、 not charge、 battery abnormal warning and external power source overvoltage warning. CHGLED is NMOS Open Drain output, which can be showed by using a current limit resistance drive a light-emitting diode and is printed in the table:

| Status | Active | Description |
| :-- | :-- | :-- |
| In charge | Output low |  |
| Not in charge | High Z | Charger goes into battery active mode, or battery <br> under/over temperature |
| Battery abnormal | $25 \%$ duty 1 Hz flip | External power supply input voltage is over |
| Over voltage | $25 \%$ duty 4 Hz flip |  |

## Battery temperature detect

In charge or use, AXP192 can gauge temperature though a temperature sensitive resistance connected with TS pin. The circuit is showed as follows:

# Page 26

![img-17.jpeg](img-17.jpeg)

In this figure, VTH/VTL is high temperature and low temperature threshold, which are set by REG38H/39H/3CH/3DH,VTE=0.2V. Use 10Kohm on $25^{\circ} \mathrm{C}$. accuracy is $1 \%$ NTC temperature sensitive resistance by suggest. AXP192 output constant current in TS pin, which can be set to $20 \mathrm{uA} \cdot 40 \mathrm{uA} \cdot 60 \mathrm{uA}$ or 80 uA (see REG84H) according to different NTC resistance. Through the temperature sensitive resistance, we can get a voltage , AXP192 use ADC to convert voltage to digital signal, and compare with the set value, sends IRQ or pause charge.

If the battery doesn't have a temperature sensitive resistance, TS pin can be connected into ground, then AXP192 disable battery temperature detect function.

# Battery detect 

AXP192 can check battery is presence or not, and set in register(see REG01H) and sends out IRQ13、 IRQ14.

Battery detect function can be enable or disable by Host(see REG32H).

### 9.4. Backup Battery

AXP192 supports backup battery use and charge. When main power supplies(BAT/ACIN/VBUS) don't exist, LDO1 input source is backup battery, to keep part of circuits such as system real time clock.

When main power supplies exist, charge backup battery though setting REG35H[7], whose target voltage is 3.0 V (which can be set by REG35H[6:5]) . default charge current is 200 uA (which can be set by REG35H[1:0]).

# Page 27

# 9.5. Multi-Outputs

AXP192's multi-output is showed as follows:

|  Output | Type | Application example | Drive ability  |
| --- | --- | --- | --- |
|  DCDC1 | BUCK | $3.3 \mathrm{~V} \mathrm{I} / \mathrm{O}$ | 1200 mA  |
|  DCDC2 | BUCK | 1.25 Vcore | 1600 mA  |
|  DCDC3 | BUCK | 2.5 Vddr | 700 mA  |
|  LDO1 | LDO | RTC | 30 mA  |
|  LDO2 | LDO | Analog/FM | 200 mA  |
|  LDO3 | LDO | 1.8 V HDMI | 200 mA  |
|  $\mathrm{LDO}_{100}$ | LDO | Vmic | 50 mA  |

AXP192 has integrated 3 Buck DC-DC converters, 4 low dropout linear regulator, multi-startup time sequence and control mode. DC-DC switch frequency is 1.5 MHz by default, which can be set by register, outside circuit use small inductance and capacitance. 3 DC-DC converters can be set to PWM mode or auto mode (that AXP192 automatically switches based on load), see "REG80H".

## DC-DC1/2/3

DCDC1/3 output voltage range is $0.7-3.5 \mathrm{~V}$, DCDC2 output voltage range is $0.7-2.275 \mathrm{~V}$, which can be set by register(see "REG23H 26H 27H 29H").

DCDC1/2/3 recommend output capacitance is 10uF X7R ceramics capacitance. When output voltage is set to higher than $2.5 \mathrm{~V}, 2.2 \mathrm{uH}$ inductance is a recommendation. But under $2.5 \mathrm{~V}, 4.7 \mathrm{uH}$ inductance is a recommendation, whose saturation current should be greater than $50 \%$ of max load current.

Recommend inductance and capacitance is showed as follows:

|  Inductance |  |   |
| --- | --- | --- |
|  Type | Current spec | Current internal resistance  |
|  Murata LQH55PN2R2NR0 | $2100 \mathrm{~mA} @ 2.2 \mathrm{uH}$ | 30 mOhm  |
|  Murata LQH55PN4R7NR0 | $1400 \mathrm{~mA} @ 4.7 \mathrm{uH}$ | 60 mOhm  |
|  Murata LQH44PN2R2MP0 | $2000 \mathrm{~mA} @ 2.2 \mathrm{uH}$ | 49 mOhm  |
|  Murata LQH44PN4R7MP0 | $1700 \mathrm{~mA} @ 2.2 \mathrm{uH}$ | 80 mOhm  |
|  TDK VLF5010ST-2R2M2R3 | $2700 \mathrm{~mA} @ 2.2 \mathrm{uH}$ | 41 mOhm  |
|  TDK VLF5014ST-4R7M1R7 | $1700 \mathrm{~mA} @ 4.7 \mathrm{uH}$ | 98 mOhm  |
|  TDK SLF6045T-4R7N2R4-3PF | $2400 \mathrm{~mA} @ 4.7 \mathrm{uH}$ | 27 mOhm  |
|  Capacitance. |  |   |
|  Type | Temperature characteristic | Tolerance  |
|  TDK C2012XSR0J475K | X5R/X7R | $10 \% @ 4.7 \mathrm{uF}$  |
|  TDK C2012XSR0J106K | X5R/X7R | $10 \% @ 10 \mathrm{uF}$  |
|  Murata GRM31E71A475K | X7R | $10 \% @ 4.7 \mathrm{uF}$  |
|  Murata GRM21E71A106K | X7R | $10 \% @ 10 \mathrm{uF}$  |
|  Murata GRM31E71A106K | X7R | $10 \% @ 10 \mathrm{uF}$  |

# Page 28

LDO1 always on, which can supply power to real time clock of system, and its output current is 30 mA .

LDO2/3
LDO2/3 low-noise LDO, which can supply power to analog circuits of application system, and its output current is $200 \mathrm{~mA}_{\text {。 }}$
$\mathrm{LDO}_{100}$
$\mathrm{LDO}_{100}$ use low-noise design also, its output current is $50 \mathrm{~mA}_{\text {。 }}$

# Soft Start 

All of DC-DC and LDO support soft start to avoid the pulse current when startup.

## Self-diagnose: load control and current limit protection

All DC-DC converters and LDO's have the function of load control and current limit protection. When load current is over than its ability, output voltage will drop. When one of the 3 DC-DC output voltages is lower than $85 \%$ of the set value, AXP192 is powered off automatically. Meanwhile the system can record which power rail makes system powered off(see REG46H[5:2]), and sends out IRQ.

All DC-DC don't need Scotty diode and the feed back resistance circuit. If the application circuits don't use any DC-DC, just floating LX pin, but the VIN and PGND should connect as normal.

### 9.6. Default Voltage/Timing Setting

The default value of the output voltage or timing sequence can be customized as applications need.

Timing sequence: there are 8 steps, $0-7$, the eighth step means that not enable as default, step 0 to 6 means the first to seventh, the time interval between ever step can be set to $1,4,16 \mathrm{mS}$.

Default voltage: The DCDC/LDO default can be set from 1 V to 3.3 V .

### 9.7. Signal Capture

The multi channel 12-bit ADC of AXP192 not only measure the cell voltage but also measure the cell current and external power source voltage and current, meanwhile internally integrates with the batteries' charge-discharge coulomb-counter. According to these data, Host can calculate accurately the battery power , what's more, can get the rich batteries' information like the real-time power consumption, the remaining

# Page 29

# AXP192 

## E-Powers

Enhanced single Cell Li-Battery and Power System Management IC
battery power, the progress of charging, the remaining time to work and the remaining time to charge completely and etc.

Enable or disable ADC and sample rate can be set through registers REG82H, 83H, 84H, and the result will be put into corresponding register, referred to ADC data register class of instructions. Among that the input range of GPIO can set through the register REG85H. whether the direction of the battery current is charging or discharging indicated by register REG00H[2].

| Channel | 000 H | STEP | FFFH |
| :--: | :--: | :--: | :--: |
| Battery Voltage | 0 mV | $1.1 \mathrm{mV}$ | 4.5045 V |
| Bat discharge current | 0 mA | 0.5 mA | 4.095 A |
| Bat charge current | 0 mA | 0.5 mA | 4.095 A |
| ACIN volatge | 0 mV | 1.7 mV | 6.9615 V |
| ACIN current | 0 mA | 0.625 mA | 2.5594 A |
| VBUS voltage | 0 mV | 1.7 mV | 6.9615 V |
| VBUS current | 0 mA | 0.375 mA | 1.5356 A |
| Internal temperature | $-144.7^{\circ} \mathrm{C}$ | $0.1^{\circ} \mathrm{C}$ | $264.8^{\circ} \mathrm{C}$ |
| APS voltage | 0 mV | 1.4 mV | 5.733 V |
| TS pin input | 0 mV | 0.8 mV | 3.276 V |
| GPIOO | $0,0.7 \mathrm{~V}$ | 0.5 mV | $2.0475 / 2.7475 \mathrm{~V}$ |
| GPIO1 | $0,0.7 \mathrm{~V}$ | 0.5 mV | $2.0475 / 2.7475 \mathrm{~V}$ |
| GPIO2 | $0,0.7 \mathrm{~V}$ | 0.5 mV | $2.0475 / 2.7475 \mathrm{~V}$ |
| GPIO[3] | $0,0.7 \mathrm{~V}$ | 0.5 mV | $2.0475 / 2.7475 \mathrm{~V}$ |

### 9.8. Multi-Function Pin Description

## GPIO[7:0]

Use as GPIO[7:0]. ADC Input (to monitor external signal). LDO. PWM, and so on., see detail in the description of REG90H-9FH.

## N_RSTO

Use as LDO1 status signal (up to LDO1) or GPIO5, see detail in the description of REG9EH.

## CHGLED

Use as GPO and warning signal such as charge status, over temperature/voltage warning and so on, see detail in the description of REG32H.

# Page 30

# 9.9. Timer 

AXP192 includes an internal timer, which can change the timer through setting register REG8AH[6:0] whose LSB is one Minute, reset the timer after timeout.

### 9.10. TWSI and IRQ

![img-18.jpeg](img-18.jpeg)

Figure 9-1. Single Read and Write

# Page 31

![img-19.jpeg](img-19.jpeg)

Enhanced single Cell Li-Battery and Power System Management IC
![img-20.jpeg](img-20.jpeg)

Figure 9-2. Multi Read and Write

Host can access registers through TWSI, its time sequence is as illustrated in shown picture, support standard 100 KHz or 400 KHz , and the maximum speed is up to 1.2 MHz , while it supports read and write operation, that device address 69 H is to read and 68 H to write. In certain cases, AXP192 reminds Host through pulling down the interrupt mechanism of IRQ, and puts the IRQ status into corresponding register(refer to register REG44H, register REG45H, register REG47H), and cancel the interruption by adding 1 to the appropriate register. IRQ output raise( increase its resistance by 51 k from outside), when there are no events of interruption. Each IRQ can be blocked through IRQ register(refer to register REG40H, register REG41H, register REG42H, REG43H).

| Location | IRQ | Description | Location | IRQ | Description |
| :--: | :--: | :--: | :--: | :--: | :--: |
| REG44H[7] | IRQ1 | ACIN overvoltage | REG 46H[7] | IRQ16 | IC internal over temperature |
| REG44H[6] | IRQ2 | ACIN insert | REG 46H[6] | IRQ17 | Charge current not enough |
| REG 44H[5] | IRQ3 | ACIN remove | REG 46H[5] | IRQ18 | DCDC1 under voltage |
| REG 44H[4] | IRQ4 | VBUS overvoltage | REG 46H[4] | IRQ19 | DCDC2 under voltage |
| REG 44H[3] | IRQ5 | VBUS insert | REG 46H[3] | IRQ20 | DCDC3 under voltage |
| REG 44H[2] | IRQ6 | VBUS remove | REG 46H[2] |  | Reserved |
| REG 44H[1] | IRQ7 | VBUS valid but lower than $\mathrm{V}_{\text {HOLD }}$ | REG 46H[1] | IRQ22 | Short time key press |
| REG 44H[0] |  | Reserved | REG 46H[0] | IRQ23 | Long time key press |
| REG 45H[7] | IRQ8 | Battery is present | REG 47H[7] | IRQ24 | Power on by N_OE |
| REG 45H[6] | IRQ9 | Battery not present | REG 47H[6] | IRQ25 | Power off by N_OE |

# Page 32

# AXP192 

Enhanced single Cell Li-Battery and Power System Management IC

| REG 45H[5] | IRQ10 | Into battery active <br> mode | REG 47H[5] | IRQ26 | VBUS valid |
| :--: | :--: | :--: | :--: | :--: | :--: |
| REG 45H[4] | IRQ11 | Quit battery active <br> mode | REG 47H[4] | IRQ27 | VBUS invalid |
| REG 45H[3] | IRQ12 | Charging | REG 47H[3] | IRQ28 | VBUS Session Valid |
| REG 45H[2] | IRQ13 | Charge finished | REG 47H[2] | IRQ29 | VBUS Session End |
| REG 45H[1] | IRQ14 | Battery over <br> temperature | REG 47H[1] |  | Reserved |
| REG 45H[0] | IRQ15 | Battery under <br> temperature | REG 47H[0] | IRQ30 | Under voltage warning |

### 9.11. Registers

### 9.11.1. power supply control class

| Location | Description | R/W | Default |
| :--: | :--: | :--: | :--: |
| 00 | Power supply status register | R |  |
| 01 | Power supply mode/charging status register | R |  |
| 04 | OTG VBUS status register | R |  |
| 06-09 | Data buffer register | R/W | F0/DF/D0/FF |
| 10 | EXTEN \& DC-DC2 switch register | R/W | X5H |
| 12 | DC-DC1/3 \& LDO2/3switch register | R/W | XFH |
| 23 | DC-DC2 voltage set register | R/W | 16 H |
| 25 | DC-DC2 voltage slope set register | R/W | 00 H |
| 26 | DC-DC1voltage set register | R/W | 68 H |
| 27 | DC-DC3 voltage set register | R/W | 48 H |
| 28 | LDO2/3 voltage set register | R/W | CFH |
| 30 | VBUS-IPSOUT access set register | R/W | 60 H |
| 31 | $\mathrm{V}_{\text {OFF }}$ power off voltage set register | R/W | X3H |
| 32 | Power off、 battery detect、CHGLED control register | R/W | 46 H |
| 33 | Charging control register1 | R/W | C8H |
| 34 | Charging control register2 | R/W | 41 H |
| 35 | Backup battery charging control register | R/W | 22 H |
| 36 | PEK parameter set register | R/W | 5DH |
| 37 | DCDC switch frequency set register | R/W | 08 H |
| 38 | Battery charging under temperature warning set register | R/W | A5H |
| 39 | Battery charging over temperature warning set register | R/W | 1 FH |
| 3 A | APS under voltage Level1 set register | R/W | 68 H |
| 3B | APS under voltage Level2 set register | R/W | 5 FH |

# Page 33

# AXP192

Enhanced single Cell Li-Battery and Power System Management IC

|  3C | Battery discharging under temperature warning set register | R/W | FCH  |
| --- | --- | --- | --- |
|  3D | Battery discharging over temperature warning set register | R/W | 16 H  |
|  80 | DCDC mode set register | R/W | EOH  |
|  82 | ADC enable set register 1 | R/W | 83 H  |
|  83 | ADC enable set register 2 | R/W | 80 H  |
|  84 | ADC sample frequency set, TS pin control register | R/W | 32 H  |
|  85 | GPIO [3:0] input range set register | R/W | XOH  |
|  8A | Timer control register | R/W | 00 H  |
|  8B | VBUS monitor set register | R/W | 00 H  |
|  8F | Over temperature power off control register | R/W | 01 H  |

### 9.11.2. GPIO control class

|  Location | Description | R/W | Default  |
| --- | --- | --- | --- |
|  90 | GPIOO control register | R/W | 07 H  |
|  91 | GPIOO LDO mode output voltage set register | R/W | AOH  |
|  92 | GPIO1 control register | R/W | 07 H  |
|  93 | GPIO2 control register | R/W | 07 H  |
|  94 | GPIO[2:0] signal status register | R/W | 00 H  |
|  95 | GPIO[4:3] function control register | R/W | 00 H  |
|  96 | GPIO[4:3] signal status register | R/W | 00 H  |
|  97 | GPIO[2:0] pull down control register | R/W | 00 H  |
|  98 | PWM1 frequency set register | R/W | 00 H  |
|  99 | PWM1 duty ratio set register 1 | R/W | 16 H  |
|  9A | PWM1 duty ratio set register 2 | R/W | 08 H  |
|  9B | PWM2 frequency set register | R/W | 00 H  |
|  9C | PWM2 duty ratio set register 1 | R/W | 16 H  |
|  9D | PWM2 duty ratio set register 2 | R/W | 08 H  |
|  9E | GPIOS control register | R/W | 20 H  |

### 9.11.3. IRQ control class

|  Location | Description | R/W | Default  |
| --- | --- | --- | --- |
|  40 | IRQ enable control register 1 | R/W | D8H  |
|  41 | IRQ enable control register 2 | R/W | FFH  |
|  42 | IRQ enable control register 3 | R/W | 38 H  |
|  43 | IRQ enable control register 4 | R/W | C1H  |

# Page 34

# AXP192

Enhanced single Cell Li-Battery and Power System Management IC

|  44 | IRQ status register 1 | R/W | 00 H  |
| --- | --- | --- | --- |
|  45 | IRQ status register 2 | R/W | 00 H  |
|  46 | IRQ status register 3 | R/W | 00 H  |
|  47 | IRQ status register 4 | R/W | 00 H  |

### 9.11.4. ADC data class

|  Location | Description | R/W  |
| --- | --- | --- |
|  56 | ACIN voltage ADC data high 8 bit | R  |
|  57 | ACIN voltage ADC data low 4 bit | R  |
|  58 | ACIN current ADC data high 8 bit | R  |
|  59 | ACIN current ADC data low 4 bit | R  |
|  5 A | VBUS voltage ADC data high 8 bit | R  |
|  5B | VBUS voltage ADC data low 4 bit | R  |
|  5 C | VBUS current ADC data high 8 bit | R  |
|  5D | VBUS current ADC data low 4 bit | R  |
|  5 E | AXP192 internal temperature monitor ADC data High 8 bit | R  |
|  5 F | AXP192 internal temperature monitor ADC data low 4 bit | R  |
|  62 | TS input ADC data High 8 bit, monitor battery temperature by default | R  |
|  63 | TS input ADC data low 4 bit, monitor battery temperature by default | R  |
|  64 | GPIOO voltage ADC data high 8 bit | R  |
|  65 | GPIOO voltage ADC data low 4 bit | R  |
|  66 | GPIO1 voltage ADC data high 8 bit | R  |
|  67 | GPIO1 voltage ADC data low 4 bit | R  |
|  68 | GPIO2 voltage ADC data high 8 bit | R  |
|  69 | GPIO2 voltage ADC data low 4 bit | R  |
|  6 A | GPIO[3] voltage ADC data high 8 bit | R  |
|  6B | GPIO[3] voltage ADC data low 4 bit | R  |
|  70 | Battery instantaneous power high 8 bit | R  |
|  71 | Battery instantaneous power middle 8 bit | R  |
|  72 | Battery instantaneous power low 8 bit | R  |
|  78 | Battery voltage high 8 bit | R  |
|  79 | Battery voltage low 4 bit | R  |
|  7 A | Battery charging current high 8 bit | R  |
|  7B | Battery charging current low 5 bit | R  |
|  7 C | Battery discharging current high 8 bit | R  |
|  7D | Battery discharging current low 5 bit | R  |
|  7 E | APS voltage high 8 bit | R  |
|  7 F | APS voltage low 4 bit | R  |

# Page 35

| Location | Description | R/W | Default |
| :--: | :--: | :--: | :--: |
| B0 | Battery charging coulomb counter data register 3 | R/W | 00 H |
| B1 | Battery charging coulomb counter data register 2 | R/W | 00 H |
| B2 | Battery charging coulomb counter data register 1 | R/W | 00 H |
| B3 | Battery charging coulomb counter data register 0 | R/W | 00 H |
| B4 | Battery discharging coulomb counter data register 3 | R/W | 00 H |
| B5 | Battery discharging coulomb counter data register 2 | R/W | 00 H |
| B6 | Battery discharging coulomb counter data register 1 | R/W | 00 H |
| B7 | Battery discharging coulomb counter data register 0 | R/W | 00 H |
| B8 | Coulomb counter control register | R/W | 00 H |

# 9.11.5. REG 00H: power supply status 

| Bit | Description | R/W |
| :--: | :-- | :--: |
| 7 | ACIN present indicator <br> 0:ACIN not present; 1:ACIN present | R |
| 6 | Indicate whether ACIN is valid or not | R |
| 5 | VBUS present indicate <br> 0:VBUS not present; 1:VBUS present | R |
| 4 | Indicate whether VBUS is valid or not | R |
| 3 | Indicate VBUS is above V, before insert | R |
| 2 | Indicate battery current direction <br> 0: battery discharging; 1: battery charging | R |
| 1 | Indicate whether ACIN and VBUS input pin is in short on PCB or not | R |
| 0 | Indicate trigger boot by ACIN/ VBUS or not <br> 0: not be ACIN/VBUS: 1: be ACIN/VBUS | R |

### 9.11.6. REG 01H: power supply work mode and charging status indicator

| Bit | Description | R/W |
| :--: | :-- | :--: |
| 7 | Indicate whether AXP192 is over temperature or not <br> 0: not over temperature; 1: over temperature | R |
| 6 | Charging indicate <br> 0: not be charging or charge finished; 1:charging | R |
| 5 | Battery present status indicator <br> 0: no battery connects to AXP192; 1:battery has connected to AXP192 | R |
| 4 | Reserved | R |

# Page 36

|  3 | Indicate whether battery goes into active mode or not
0:not in battery active mode; 1:in battery active mode | R  |
| --- | --- | --- |
|  2 | Indicate whether charging current is less than expected current
0: actual charging current equal to expected current; 1: actual charging current less than
expected current | R  |
|  $1-0$ | Reserved | R  |

# 9.11.7. REG 04H:USB OTG VBUS status indicator

|  Bit | Description | R/W  |
| --- | --- | --- |
|  $7-3$ | Reserved |   |
|  2 | Indicate whether VBUS is valid or not, 1: valid | R  |
|  1 | Indicate whether VBUS Session A/B is valid or not, 1: valid | R  |
|  0 | Indicate Session End status, 1: valid | R  |

### 9.11.8. REG 06-09H:data buffer 0-3

Notice: if either external power supply, battery or backup battery is present, this 4 byte data will be saved, not affected by the power on/off status of the system.

### 9.11.9. REG 10H:EXTEN \& DC-DC2 output control

Default: XXH

|  Bit | Description |  | R/W | Default  |
| --- | --- | --- | --- | --- |
|  $7-3$ | Reserved |  |  |   |
|  2 | EXTEN switch control | 0:disable; 1:enable | RW | X  |
|  1 | Reserved |  |  |   |
|  0 | DC-DC2 switch control | 0:disable; 1:enable | RW | X  |

Notice: X mean its value considered by condition

### 9.11.10. REG 12H:DC-DC1/3 \& LDO2/3 output control

Default: XEH

|  Bit | Description |  | R/W | Default  |
| --- | --- | --- | --- | --- |
|  $7-4$ | Reserved |  |  |   |
|  3 | LDO3 switch control | 0:disable; 1:enable | RW | X  |

# Page 37

# AXP192

Enhanced single Cell Li-Battery and Power System Management IC

|  2 | LDO2 switch control |  | RW | X  |
| --- | --- | --- | --- | --- |
|  1 | DC-DC3 switch control |  | RW | X  |
|  0 | DC-DC1 switch control |  | RW | X  |

### 9.11.11. REG 23H:DC-DC2 output voltage set

Default: 16 H

|  Bit | Description |  | R/W | Default  |
| --- | --- | --- | --- | --- |
|  7-6 | Reserved |  |  |   |
|  5 | DC-DC2 output voltage Bit5 | $0.7-2.275 \mathrm{~V}, 25 \mathrm{mV} /$ step | RW | X  |
|  4 | DC-DC2 output voltage Bit4 |  | RW | X  |
|  3 | DC-DC2 output voltage Bit3 |  | RW | X  |
|  2 | DC-DC2 output voltage Bit2 |  | RW | X  |
|  1 | DC-DC2 output voltage Bit1 |  | RW | X  |
|  0 | DC-DC2 output voltage Bit0 |  | RW | X  |

### 9.11.12. REG 25H: DC-DC2 dynamic voltage parameter set

Default: 00 H

|  Bit | Description |  | R/W | Default  |
| --- | --- | --- | --- | --- |
|  7-3 | Reserved |  |  |   |
|  2 | DC-DC2 VRC enable control
0:enable; 1:disable |  | RW | 0  |
|  1 | Reserved |  | RW | 0  |
|  0 | DC-DC2 VRC voltage slope control | $0: 25 \mathrm{mV} / 15.625 \mathrm{us}=1.6 \mathrm{mV} / \mathrm{us}$
$1: 25 \mathrm{mV} / 31.250 \mathrm{us}=0.8 \mathrm{mV} / \mathrm{us}$ | RW | 0  |

### 9.11.13. REG 26H:DC-DC1 output voltage set

Default: 68 H

|  Bit | Description |  | R/W | Default  |
| --- | --- | --- | --- | --- |
|  7 | Reserved |  |  |   |
|  6 | DC-DC1 output voltage Bit6 | $0.7-3.5 \mathrm{~V}, 25 \mathrm{mV} /$ step | RW | X  |
|  5 | DC-DC1 output voltage Bit5 |  | RW | X  |
|  4 | DC-DC1 output voltage Bit4 |  | RW | X  |
|  3 | DC-DC1 output voltage Bit3 |  | RW | X  |

# Page 38

# AXP192

Enhanced single Cell Li-Battery and Power System Management IC

|  2 | DC-DC1 output voltage Bit2 |  | RW | X  |
| --- | --- | --- | --- | --- |
|  1 | DC-DC1 output voltage Bit1 |  | RW | X  |
|  0 | DC-DC1 output voltage Bit0 |  | RW | X  |

### 9.11.14. REG 27H:DC-DC3 output voltage set

Default: 48 H

|  Bit | Description |  | R/W | Default  |
| --- | --- | --- | --- | --- |
|  7 | Reserved |  |  |   |
|  6 | DC-DC3 output voltage Bit6 | $0.7-3.5 \mathrm{~V}, 25 \mathrm{mV} /$ step | RW | X  |
|  5 | DC-DC3 output voltage Bit5 |  | RW | X  |
|  4 | DC-DC3 output voltage Bit4 |  | RW | X  |
|  3 | DC-DC3 output voltage Bit3 |  | RW | X  |
|  2 | DC-DC3 output voltage Bit2 |  | RW | X  |
|  1 | DC-DC3 output voltage Bit1 |  | RW | X  |
|  0 | DC-DC3 output voltage Bit0 |  | RW | X  |

### 9.11.15. REG 28H:LDO2/3 output voltage set

Default: CFH

|  Bit | Description |  | R/W | Default  |
| --- | --- | --- | --- | --- |
|  7 | LDO2 output voltage Bit3 | $1.8-3.3 \mathrm{~V}, 100 \mathrm{mV} /$ step | RW | X  |
|  6 | LDO2 output voltage Bit2 |  | RW | X  |
|  5 | LDO2 output voltage Bit1 |  | RW | X  |
|  4 | LDO2 output voltage Bit0 |  | RW | X  |
|  3 | LDO3 output voltage Bit3 | $1.8-3.3 \mathrm{~V}, 100 \mathrm{mV} /$ step | RW | X  |
|  2 | LDO3 output voltage Bit2 |  | RW | X  |
|  1 | LDO3 output voltage Bit1 |  | RW | X  |
|  0 | LDO3 output voltage Bit0 |  | RW | X  |

### 9.11.16. REG 30H:VBUS-IPSOUT access management

Default:6XH

|  Bit | Description | R/W | Default  |
| --- | --- | --- | --- |
|  7 | When VBUS is valid, VBUS-IPSOUT access choose control signal
0 :be enable or not by N_VBUSEN pin | RW | 0  |

# Page 39

# AXP192

Enhanced single Cell Li-Battery and Power System Management IC

|   | 1:VBUS-IPSOUT access is enable, regardless of N_VBUSEN status |  |  |  |   |
| --- | --- | --- | --- | --- | --- |
|  6 | VBUS V $_{\text {HOLD }}$ limit voltage control
0 : not limited; 1 : limited |  |  | RW | 1  |
|  5 | $\mathrm{V}_{\text {HOLD }}$ Bit 2 | $000: 4.0 \mathrm{~V}$ : | $001: 4.1 \mathrm{~V}$ : | $010: 4.2 \mathrm{~V}$ | RW  |
|  4 | $\mathrm{V}_{\text {HOLD }}$ Bit 1 | $011: 4.3 \mathrm{~V}$ : | $100: 4.4 \mathrm{~V}$ : | $101: 4.5 \mathrm{~V}$ | RW  |
|  3 | $\mathrm{V}_{\text {HOLD }}$ Bit 0 | $110: 4.6 \mathrm{~V}$ : | $111: 4.7 \mathrm{~V}$ |  | RW  |
|  2 | Reserved |  |  |  |   |
|  1 | VBUS limit current control enable signal
0 : disable; 1 :enable |  |  | RW | X  |
|  0 | When VBUS limit current control is enable, choose limit value
0:500mA: 1:100mA |  |  | RW | 0  |

### 9.11.17. REG 31H:V $_{\text {OFF }}$ power off voltage set

Default:X3H

|  Bit | Description |  |  |  | R/W | Default  |
| --- | --- | --- | --- | --- | --- | --- |
|  $7-3$ | Reserved |  |  |  |  |   |
|  2 | $\mathrm{V}_{\text {OFF }}$ Bit2 | $000-2.6 \mathrm{~V}$ : | $001-2.7 \mathrm{~V}$ : | $010-2.8 \mathrm{~V}$ : | RW | 0  |
|  1 | $\mathrm{V}_{\text {OFF }}$ Bit1 | $011-2.9 \mathrm{~V}$ : | $100-3.0 \mathrm{~V}$ : | $101-3.1 \mathrm{~V}$ : | RW | 1  |
|  0 | $\mathrm{V}_{\text {OFF }}$ Bit0 | $110-3.2 \mathrm{~V}$ : | $111-3.3 \mathrm{~V}$ |  | RW | 1  |

### 9.11.18. REG 32H:power off、 battery check and CHGLED pin control

Default:46H

|  Bit | Description |  | R/W | Default  |
| --- | --- | --- | --- | --- |
|  7 | Power off control
1: close AXP192 output |  | RW | 0  |
|  6 | Battery monitor function set: 0:disable; 1:enable |  | RW | 1  |
|  $5-4$ | CHGLED pin function | 00: high Z
01: $25 \% 1 \mathrm{~Hz}$ flip
10: $25 \% 4 \mathrm{~Hz}$ flip
11: output low | RW | 00  |
|  3 | CHGLED pin function | 0: control by charge
1: control by REG 32HBit[5:4] | RW | 0  |
|  2 | Reserved |  |  |   |
|  $1-0$ | After N_OE from low to high, AXP192
power off delay time | 00: 0.5S: 01: 1S:
10: 2S: 11: 3S | RW | 10  |

# Page 40

# 9.11.19. REG 33H: charging control 1

Default: C8H

|  Bit | Description |  | R/W | Default  |
| --- | --- | --- | --- | --- |
|  7 | Charging enable control bit, include internal access and external access
0:disable; 1:enable |  | RW | 1  |
|  6:5 | Charging target voltage
00:4.1V; 01:4.15V; 10:4.2V; 11:4.36V |  | RW | 10  |
|  4 | Charge finished current
0: when charging current is less than $10 \%$ set value, finish charging
1: when charging current is less than $15 \%$ set value, finish charging |  | RW | 0  |
|  $3-0$ | Internal charging current
0000:100mA; 0001:190mA; 0010:280mA; 0011:360mA;
0100:450mA; 0101:550mA; 0110:630mA; 0111:700mA;
1000:780mA; 1001:880mA; 1010:960mA; 1011:1000mA;
1100:1080mA; 1101:1160mA; 1110:1240mA; 1111:1320mA |  | RW | 1000  |

### 9.11.20. REG 34H: charging control 2

Default:41H

|  Bit | Description |  | R/W | Default  |
| --- | --- | --- | --- | --- |
|  7 | Pre-charge overtime Bit1 | 00:30 min; 01:40min; | RW | 0  |
|  6 | Pre-charge overtime Bit0 | 10:50min; 11:60min | RW | 1  |
|  $5-3$ | External access charging current
Range 300-1000mA, 100mA/step, Default:300mA |  | RW | 000  |
|  2 | External access enable set while charging
0:disable; 1:enable |  | RW | 0  |
|  1 | In constant current mode overtime
Bit1 | 00: 7Hours; 01: 8Hours;
10: 9Hours; 11: 10Hours | RW | 0  |
|  0 | In constant current mode overtime
Bit0 |  | RW | 1  |

### 9.11.21. REG 35H: Backup battery charge control

Default: 22 H

|  Bit | Description | R/W | Default  |
| --- | --- | --- | --- |
|  7 | Backup battery charging enable control
0:disable; 1:enable | RW | 0  |

# Page 41

# 9.11.22. REG 36H:PEK press key parameter set 

Default: 5DH

| Bit | Description | R/W | Default |
| :-- | :-- | :-- | :-- |
| 7 -0 | Reserved |  |  |
| 3 | DC-DC switch frequency Bit 3 | $5 \%$ per step,default1.5MHz | RW | 1 |
| 2 | DC-DC switch frequency Bit 2 | RW | 0 |
| 1 | DC-DC switch frequency Bit 1 | RW | 0 |
| 0 | DC-DC switch frequency Bit 0 | RW | 0 |

### 9.11.24. REG 38H:V ${ }_{\text {ITF-charge }}$ battery charging low temperature threshold set

Default:ASH

| Bit | Description | R/W | Default |
| :--: | :--: | :--: | :--: |
| 7-0 | In charge, low temperature | $\mathrm{M}^{*} 10 \mathrm{H}$, when $\mathrm{M}=\mathrm{A} 5 \mathrm{H}, \mathrm{V}_{\text {LTF-charge }}=2.112 \mathrm{~V}$, | RW | ASH |

# Page 42

# AXP192

Enhanced single Cell Li-Battery and Power System Management IC

|   | threshold, M | range :0V~3.264V |   |
| --- | --- | --- | --- |
|  |   |   |   |

## 9.11.25. REG 39H:V_{HTF-charge} battery charging high temperature threshold set

Default:1FH

|  Bit | Description | R/W | Default  |
| --- | --- | --- | --- |
|  7-0 | In charge, high temperature threshold, N | N*10H, when N=1FH, V_{HTF-charge} =0.397V, range:0V~3.264V | RW  |

V_{HTF-charge} = N *10H * 0.0008V

## 9.11.26. REG 3AH:APS low voltage level 1

Default:68H

|  Bit | Description | R/W | Default  |
| --- | --- | --- | --- |
|  7-0 | APS low voltage level 1 | RW | 68H  |

## 9.11.27. REG 3BH:APS low voltage level 2

Default:5FH

|  Bit | Description | R/W | Default  |
| --- | --- | --- | --- |
|  7-0 | APS low voltage level 2 | RW | 5FH  |

REG3AH、REG3BH corresponding APS voltage is:(suppose register value is n):

Vwarning = 2.8672 + 1.4mV * n * 4

## 9.11.28. REG 3CH:V_{ITF-discharge} battery discharging low temperature threshold set

Default: FCH

|  Bit | Description | R/W | Default  |
| --- | --- | --- | --- |
|  7-0 | In discharge, low temperature threshold set, M | M*10H, when M=FCH, V_{ITF-discharge} =3.226V; range 0V~3.264V | RW  |

V_{ITF-discharge} = M *10H * 0.0008V

## 9.11.29. REG 3DH:V_{HTF-discharge} battery discharging high temperature threshold set

Default:16H

|  Bit | Description | R/W | Default  |
| --- | --- | --- | --- |
|  |   |   |   |

AXP192 Datasheet(Revision1.1) Copyright © 2015 X-Powers Limited. All Rights Reserved. Page 42/53

# Page 43

# AXP192

Enhanced single Cell Li-Battery and Power System Management IC

|  7-0 | In discharge, high temperature
threshold set, N | N*10H, when N=16H, VHTF-discharge 0.282V;
range 0V~3.264V | RW | 16H  |
| --- | --- | --- | --- | --- |
|  |   |   |   |   |

VHTF-discharge = N * 10H * 0.0008V

## 9.11.30. REG 80H:DC-DC work mode choose

### Default:EOH

|  Bit | Description | R/W | Default  |
| --- | --- | --- | --- |
|  7-4 | Reserved |  |   |
|  3 | DC-DC1 work mode control | 0:PFM/PWM auto switch
1: fixed PWM | RW  |
|  2 | DC-DC2 work mode control |  | RW  |
|  1 | DC-DC3 work mode control |  | RW  |
|  0 | Reserved |  |   |

## 9.11.31. REG 82H:ADC enable 1

### Default:83H

|  Bit | Description | R/W | Default  |
| --- | --- | --- | --- |
|  7 | Battery voltage ADC enable | 0:disable; 1:enable | RW  |
|  6 | Battery current ADC enable |  | RW  |
|  5 | ACIN voltage ADC enable |  | RW  |
|  4 | ACIN current ADC enable |  | RW  |
|  3 | VBUS voltage ADC enable |  | RW  |
|  2 | VBUS current ADC enable |  | RW  |
|  1 | APS voltage ADC enable |  | RW  |
|  0 | TS pin ADC function enable |  | RW  |

## 9.11.32. REG 83H:ADC enable 2

### Default:80H

|  Bit | Description | R/W | Default  |
| --- | --- | --- | --- |
|  7 | AXP192 internal temperature monitor
ADC enable | 0:disable; 1:enable | RW  |
|  6-4 | Reserved |  |   |
|  3 | GPIO0 ADC function enable | 0:disable; 1:enable | RW  |
|  2 | GPIO1 ADC function enable |  | RW  |
|  1 | GPIO2 ADC function enable |  | RW  |

# Page 44

# 9.11.33. REG 84H: ADC sample rate set, TS pin control

Default:32H

|  Bit | Description |  |  | R/W | Default  |
| --- | --- | --- | --- | --- | --- |
|  7 | ADC sample rate Bit 1 | $25 \times 2^{n}$ |  | RW | 0  |
|  6 | ADC sample rate Bit 0 | Sample rate: 25, 50, 100, 200 Hz |  | RW | 0  |
|  5-4 | TS pin output current set:
00:20uA: 01:40uA: 10:60uA: 11:80uA |  |  | RW | 11  |
|  3 | Reserved |  |  |  |   |
|  2 | TS pin function choose
0 :battery temperature monitor; 1 :external ADC input access |  |  | RW | 0  |
|  $1-0$ | TS pin current output set | 00:disable
01:output current when in charge
10:output when ADC sampling, to save
energy
11:always enable |  | RW | 1  |
|   |  |  |  | RW | 0  |

### 9.11.34. REG 85H: ADC input range

Default:X0H

|  Bit | Description |  | R/W | Default  |
| --- | --- | --- | --- | --- |
|  $7-4$ | Reserved |  |  |   |
|  3 | GPIO3 ADC input range | $0: 0-2.0475 \mathrm{~V}$ | RW | 0  |
|  2 | GPIO2 ADC input range | $1: 0.7-2.7475 \mathrm{~V}$ | RW | 0  |
|  1 | GPIO1 ADC input range |  | RW | 0  |
|  0 | GPIO0 ADC input range |  | RW | 0  |

### 9.11.35. REG 8AH: timer control

Default:00H

|  Bit | Description | R/W | Default  |
| --- | --- | --- | --- |
|  7 | Timer timeout
Set 1 clear this status | RW | 0  |
|  $6-0$ | Set time, unit is minute
0000000 :close this timer | RW | 0000000  |

# Page 45

# 9.11.36. REG 8BH:VBUS pin monitor SRP function control

Default:00H

|  Bit | Description | R/W | Default  |
| --- | --- | --- | --- |
|  $7-6$ | Reserved |  |   |
|  $5-4$ | VBUS valid voltage set
00:4.0V: 01:4.15V: 10:4.45V: 11:4.55V | RW | 00  |
|  3 | VBUS Valid monitor function set:0:disable, 1:enable | RW | 0  |
|  2 | VBUS Session monitor function set: 0:disable, 1:enable | RW | 0  |
|  1 | Discharge VBUS discharge function set
0: remove VBUS discharge resistor; 1: use VBUS discharge resistor | RW | 0  |
|  0 | Charge VBUS charge function set
0:remove VBUS charge resistor; 1: use VBUS charge resistor | RW | 0  |

### 9.11.37. REG 8FH: over temperature power off function set

Default:01H

|  Bit | Description | R/W | Default  |
| --- | --- | --- | --- |
|  $7-3$ | Reserved | RW | 0  |
|  2 | AXP192 internal over temperature power off function set
0:not power off; 1:power off | RW | 0  |
|  $1-0$ | Reserved |  |   |

### 9.11.38. REG 90H:GPIO0 function set

Default:07H

|  Bit | Description |  | R/W | Default  |
| --- | --- | --- | --- | --- |
|  $7-3$ | Reversed,not change |  |  |   |
|  2 | GPIOO pin function Bit 2 | 000:NMOS open drain output
001:General input function
010:low noise LDO | RW | 1  |
|  1 | GPIOO pin function Bit 1 | 100:ADC in
101: Low output | RW | 1  |
|  0 | GPIOO pin function Bit 0 | 11X: Floating | RW | 1  |

# Page 46

# 9.11.39. REG 91H:GPIO0 in LDO mode, output voltage set

Default:AOH

|  Bit | Description |  |  | R/W | Default  |
| --- | --- | --- | --- | --- | --- |
|  7-4 | GPIO0 in LDO mode, output voltage |  |  | RW | 1010  |
|   | 0000: 1.8V: | 0001: 1.9V: | 0010: 2.0V: | 0011: 2.1V: |   |
|   | 0100: 2.2V: | 0101: 2.3V: | 0110: 2.4V: | 0111: 2.5V: |   |
|   | 1000: 2.6V: | 1001: 2.7V: | 1010: 2.8V: | 1011: 2.9V: |   |
|   | 1100: 3.0V: | 1101: 3.1V: | 1110: 3.2V: | 1111: 3.3V |   |
|  3-0 | Reserved |  |  |  |   |

### 9.11.40. REG 92H:GPIO1function set

Default:07H

|  Bit | Description |  |  | R/W | Default  |
| --- | --- | --- | --- | --- | --- |
|  7-3 | Reversed, not change |  |  |  |   |
|  2 | GPIO1 pin function Bit 2 |  | 000:NMOS open drain output | RW | 1  |
|   |  |  | 001: General input function |  |   |
|   |  |  | 010:PWM1, high output is VINT |  |   |
|  1 | GPIO1 pin function Bit 1 |  | 100:ADC input | RW | 1  |
|   |  |  | 101:Low output |  |   |
|  0 | GPIO1 pin function Bit 0 |  | 11X:Floating | RW | 1  |

### 9.11.41. REG 93H:GPIO2 function set

Default:07H

|  Bit | Description |  |  | R/W | Default  |
| --- | --- | --- | --- | --- | --- |
|  7-3 | Reversed, not change |  |  |  |   |
|  2 | GPIO2 pin function Bit 2 |  | 000:NMOS open drain output | RW | 1  |
|   |  |  | 001:General input function |  |   |
|   |  |  | 010:PWM2, high output is VINT |  |   |
|  1 | GPIO2 pin function Bit 1 |  | 100:ADC input | RW | 1  |
|   |  |  | 101:Low output |  |   |
|  0 | GPIO2 pin function Bit 0 |  | 11X: Floating | RW | 1  |

# Page 47

# 9.11.42. REG 94H:GPIO[2:0] signal status set and monitor

Default:00H

|  Bit | Description |  | R/W | Default  |
| --- | --- | --- | --- | --- |
|  7 | Reserved |  | R |   |
|  6 | GPIO2 input status | 0:Low | R |   |
|  5 | GPIO1 input status | 1:High | R |   |
|  4 | GPIO0 input status |  | R |   |
|  3 | Reserved |  |  |   |
|  2 | GPIO2 output set | 0:Low, Ground NMOS open | RW | 0  |
|  1 | GPIO1 output set | 1:Float, NMOS close | RW | 0  |
|  0 | GPIO0 output set |  | RW | 0  |

### 9.11.43. REG 95H:GPIO[4:3] pin function set

Default:00H

|  Bit | Description |  | R/W | Default  |
| --- | --- | --- | --- | --- |
|  $7-4$ | Reserved |  | RW | 0  |
|  $3: 2$ | GPIO4 pin function Bit 1-0 | 00:External charge control
01:NMOS open drain output pin 4
10: General input pin 4
11:Undefine | RW | 00  |
|  $1: 0$ | GPIO3pin function Bit1-0 | 00: External charge control
01:NMOS open drain output pin 3
10: General input pin 3
11:ADC input | RW | 00  |

### 9.11.44. REG 96H: GPIO[4:3] signal status set and monitor

Default: 00 H

|  Bit | Description |  | R/W | Default  |
| --- | --- | --- | --- | --- |
|  $7-6$ | Reserved |  | R |   |
|  5 | GPIO4 input status | 0:Low | R |   |
|  4 | GPIO3 input status | 1:High | R |   |
|  $3-2$ | Reserved |  |  |   |
|  1 | GPIO4 output | 0:Low, NMOS open | RW | 0  |

# Page 48

# AXP192

Enhanced single Cell Li-Battery and Power System Management IC

|  0 | GPIO3 output | 1:Float, NMOS close | RW | 0  |
| --- | --- | --- | --- | --- |
|  |   |   |   |   |

9.11.45. REG 97H: when GPIO[2:0] is used as input, pull down set

Default: 00 H

|  Bit | Description |  | R/W | Default  |
| --- | --- | --- | --- | --- |
|  7-3 | Reserved |  |  |   |
|  2 | GPIO2 as input, pull down resistor control | 0:Remove
1:Connect | RW | 0  |
|  1 | GPIO1 as input, pull down resistor control |  | RW | 0  |
|  0 | GPIO0 as input, pull down resistor control |  | RW | 0  |

9.11.46. REG 98H:PWM1 output frequency set

Default: 00 H

|  Bit | Description |  | R/W | Default  |
| --- | --- | --- | --- | --- |
|  7-0 | PWM1 output frequency $X$ |  | RW | 00 H  |

9.11.47. REG 99H:PWM1 duty ratio set 1

Default: 16 H

|  Bit | Description |  | R/W | Default  |
| --- | --- | --- | --- | --- |
|  7-0 | PWM1 duty ratio Y1 |  | RW | 16 H  |

9.11.48. REG 9AH:PWM1 duty ratio set 2

Default: 08 H

|  Bit | Description |  | R/W | Default  |
| --- | --- | --- | --- | --- |
|  7-6 | PWM1 duty ratio Y2 |  | RW | 08 H  |

9.11.49. REG 9BH:PWM2 output frequency set

Default: 00 H

|  Bit | Description |  | R/W | Default  |
| --- | --- | --- | --- | --- |
|  |   |   |   |   |

# Page 49

# 9.11.50. REG 9CH:PWM2 duty ratio set 1

Default:16H

|  Bit | Description | R/W | Default  |
| --- | --- | --- | --- |
|  7-0 | PWM2 duty ratio Y1 | RW | 16 H  |

### 9.11.51. REG 9DH:PWM2 duty ratio set 2

Default:0BH

|  Bit | Description | R/W | Default  |
| --- | --- | --- | --- |
|  7-6 | PWM2 duty ratio Y2 | RW | 0BH  |

Notice: PWM output frequency $=2.25 \mathrm{MHz} /(\mathrm{X}+1) / \mathrm{Y} 1$ PWM output duty ratio $=\mathrm{Y} 2 / \mathrm{Y} 1$

### 9.11.52. REG 9EH: N_RSTO pin function set

Default:20H

|  Bit | Description | R/W | Default  |
| --- | --- | --- | --- |
|  7 | N_RSTO pin function
0: N_RSTO, LDO1 status monitor; 1:General I/O pin 5 | RW | 0  |
|  6 | N_RSTO as general I/O pin 5
0:NMOS open drain output; 1:General input function | RW | 0  |
|  5 | N_RSTO as output pin 5
0:Low, NMOS open; 1:Float, NMOS close | RW | 1  |
|  4 | N_RSTO as input pin 5
0:Low; 1:High | R |   |
|  3-0 | Reserved | RW | 0000  |

### 9.11.53. REG 40H:IRQ enable 1

# Page 50

Default:D8H

| Bit | Description | R/W | Default |
| :--: | :--: | :--: | :--: |
| 7 | ACIN over voltage IRQ enable | RW | 1 |
| 6 | ACIN insert IRQ enable | RW | 1 |
| 5 | ACIN remove IRQ enable | RW | 0 |
| 4 | VBUS over voltage IRQ enable | RW | 1 |
| 3 | VBUS insert IRQ enable | RW | 1 |
| 2 | VBUS remove IRQ enable | RW | 0 |
| 1 | VBUS valid, but lower than $\mathrm{V}_{\text {HOLD }}$ IRQ enable | RW | 0 |
| 0 | Reserved | RW | 0 |

# 9.11.54. REG 41H:IRQ enable 2 

Default: FFH

| Bit | Description | R/W | Default |
| :--: | :--: | :--: | :--: |
| 7 | Battery insert IRQ enable | RW | 1 |
| 6 | Battery remove IRQ enable | RW | 1 |
| 5 | Battery active mode IRQ enable | RW | 1 |
| 4 | Quit battery active mode IRQ enable | RW | 1 |
| 3 | Charging IRQ enable | RW | 1 |
| 2 | Charge finished IRQ enable | RW | 1 |
| 1 | Battery over temperature IRQ enable | RW | 1 |
| 0 | Battery under temperature IRQ enable | RW | 1 |

### 9.11.55. REG 42H:IRQ enable 3

Default:3BH

| Bit | Description | R/W | Default |
| :--: | :--: | :--: | :--: |
| 7 | AXP192 internal over temperature IRQ enable | RW | 0 |
| 6 | Charge current not enough IRQ enable | RW | 0 |
| 5 | DC-DC1 under voltage IRQ enable | RW | 1 |
| 4 | DC-DC2 under voltage IRQ enable | RW | 1 |
| 3 | DC-DC3 under voltage IRQ enable | RW | 1 |
| 2 | Reserved |  |  |
| 1 | Short time key press IRQ enable | RW | 1 |
| 0 | Long time key press IRQ enable | RW | 1 |

# Page 51

# 9.11.56. REG 43H:IRQ enable 4 

Default:C1H

| Bit | Description | R/W | Default |
| :--: | :--: | :--: | :--: |
| 7 | Power on by N_OE IRQ enable | RW | 1 |
| 6 | Power off by N_OE IRQ enable | RW | 1 |
| 5 | VBUS valid IRQ enable | RW | 0 |
| 4 | VBUS invalid IRQ enable | RW | 0 |
| 3 | VBUS Session A/B IRQ enable | RW | 0 |
| 2 | VBUS Session End IRQ enable | RW | 0 |
| 1 | Reserved | RW | 1 |
| 0 | APS under voltage IRQ enable | RW | 1 |

### 9.11.57. REG 44H:IRQ status1

Default:00H

| Bit | Description | R/W | Default |
| :--: | :--: | :--: | :--: |
| 7 | ACIN over voltage IRQ status | RW | 0 |
| 6 | ACIN insert IRQ status | RW | 0 |
| 5 | ACIN remove IRQ status | RW | 0 |
| 4 | VBUS over voltage IRQ status | RW | 0 |
| 3 | VBUS insert IRQ status | RW | 0 |
| 2 | VBUS remove IRQ status | RW | 0 |
| 1 | VBUS valid, but lower than $\mathrm{V}_{\text {HOLD }}$ IRQ status | RW | 0 |
| 0 | Reserved | RW | 0 |

### 9.11.58. REG 45H:IRQ status2

Default:00H

| Bit | Description | R/W | Default |
| :--: | :--: | :--: | :--: |
| 7 | Battery insert IRQ status | RW | 0 |
| 6 | Battery remove IRQ status | RW | 0 |
| 5 | Battery active mode IRQ status | RW | 0 |
| 4 | Quit battery active mode IRQ status | RW | 0 |
| 3 | Charging IRQ status | RW | 0 |
| 2 | Charge finished IRQ status | RW | 0 |
| 1 | battery over temperature IRQ status | RW | 0 |

# Page 52

# 9.11.59. REG 46H:IRQ status3

Default:00H

|  Bit | Description | R/W | Default  |
| --- | --- | --- | --- |
|  7 | AXP192 internal over temperature IRQ status | RW | 0  |
|  6 | Charge current not enough IRQ status | RW | 0  |
|  5 | DC-DC1 under voltage IRQ status | RW | 0  |
|  4 | DC-DC2 under voltage IRQ status | RW | 0  |
|  3 | DC-DC3 under voltage IRQ status | RW | 0  |
|  2 | Reserved |  |   |
|  1 | Short time key press IRQ status | RW | 0  |
|  0 | Long time key press IRQ status | RW | 0  |

Notice: Set 1 to any of IRQ status register will clear corresponding status.

### 9.11.60. REG 47H:IRQ status4

Default:00H

|  Bit | Description | R/W | Default  |
| --- | --- | --- | --- |
|  7 | Power on by N_OE IRQ status | RW | 0  |
|  6 | Power off by N_OE IRQ status | RW | 0  |
|  5 | VBUS valid IRQ status | RW | 0  |
|  4 | VBUS invalid IRQ status | RW | 0  |
|  3 | VBUS Session A/B IRQ status | RW | 0  |
|  2 | VBUS Session End IRQ status | RW | 0  |
|  1 | Reserved | RW | 0  |
|  0 | APS under voltage IRQ status, when APS voltage is lower than Warning
Leve2,then set 1, when is above Warning Level1, set 0. | RW | 0  |

### 9.11.61. REG B8H: Coulomb counter control

Default:00H

|  Bit | Description | R/W | Default  |
| --- | --- | --- | --- |
|  7 | Coulomb counter open/close | RW | 0  |
|  6 | Coulomb counter pause, 1: pause, then clear itself | RW | 0  |
|  5 | Clear coulomb counter control, 1:clear coulomb counter, then clear itself | RW | 0  |

# Page 53

# 10. Package 

![img-21.jpeg](img-21.jpeg)
![img-22.jpeg](img-22.jpeg)
![img-23.jpeg](img-23.jpeg)

