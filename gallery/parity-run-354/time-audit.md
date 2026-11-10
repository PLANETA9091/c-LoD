# parity session time audit (the §7b console channel)

- v6.25.5: boot markers, pins, gamerules and joins from the parity session server.log (tail 40). The #332 night-matrix class is self-diagnosing from here on.

```
4393:[18:21:18 INFO]: System chat: Incorrect argument for command
4394:gamerule doDaylightCycle false<--[HERE]
4464:[18:21:24 INFO]: System chat: Incorrect argument for command
4465:gamerule doDaylightCycle false<--[HERE]
4554:[18:21:32 INFO]: System chat: Incorrect argument for command
4555:gamerule doDaylightCycle false<--[HERE]
4621:[18:21:38 INFO]: System chat: Incorrect argument for command
4622:gamerule doDaylightCycle false<--[HERE]
4682:[18:21:44 INFO]: System chat: Incorrect argument for command
4683:gamerule doDaylightCycle false<--[HERE]
4772:[18:21:52 INFO]: System chat: Incorrect argument for command
4773:gamerule doDaylightCycle false<--[HERE]
4838:[18:21:58 INFO]: System chat: Incorrect argument for command
4839:gamerule doDaylightCycle false<--[HERE]
4910:[18:22:04 INFO]: System chat: Incorrect argument for command
4911:gamerule doDaylightCycle false<--[HERE]
4999:[18:22:12 INFO]: System chat: Incorrect argument for command
5000:gamerule doDaylightCycle false<--[HERE]
5066:[18:22:18 INFO]: System chat: Incorrect argument for command
5067:gamerule doDaylightCycle false<--[HERE]
5138:[18:22:24 INFO]: System chat: Incorrect argument for command
5139:gamerule doDaylightCycle false<--[HERE]
5227:[18:22:33 INFO]: System chat: Incorrect argument for command
5228:gamerule doDaylightCycle false<--[HERE]
5329:[18:22:42 INFO]: System chat: Incorrect argument for command
5330:gamerule doDaylightCycle false<--[HERE]
5396:[18:22:48 INFO]: System chat: Incorrect argument for command
5397:gamerule doDaylightCycle false<--[HERE]
5467:[18:22:54 INFO]: System chat: Incorrect argument for command
5468:gamerule doDaylightCycle false<--[HERE]
5556:[18:23:02 INFO]: System chat: Incorrect argument for command
5557:gamerule doDaylightCycle false<--[HERE]
5624:[18:23:08 INFO]: System chat: Incorrect argument for command
5625:gamerule doDaylightCycle false<--[HERE]
5697:[18:23:15 INFO]: System chat: Incorrect argument for command
5698:gamerule doDaylightCycle false<--[HERE]
5787:[18:23:23 INFO]: System chat: Incorrect argument for command
5788:gamerule doDaylightCycle false<--[HERE]
5860:[18:23:29 INFO]: System chat: Incorrect argument for command
5861:gamerule doDaylightCycle false<--[HERE]
5927:[18:23:35 INFO]: System chat: Incorrect argument for command
5928:gamerule doDaylightCycle false<--[HERE]
6006:[18:23:42 INFO]: System chat: Incorrect argument for command
6007:gamerule doDaylightCycle false<--[HERE]
6073:[18:23:48 INFO]: System chat: Incorrect argument for command
6074:gamerule doDaylightCycle false<--[HERE]
6146:[18:23:54 INFO]: System chat: Incorrect argument for command
6147:gamerule doDaylightCycle false<--[HERE]
6236:[18:24:02 INFO]: System chat: Incorrect argument for command
6237:gamerule doDaylightCycle false<--[HERE]
6292:[18:24:08 INFO]: System chat: Incorrect argument for command
6293:gamerule doDaylightCycle false<--[HERE]
6362:[18:24:14 INFO]: System chat: Incorrect argument for command
6363:gamerule doDaylightCycle false<--[HERE]
6463:[18:24:23 INFO]: System chat: Incorrect argument for command
6464:gamerule doDaylightCycle false<--[HERE]
6565:[18:24:32 INFO]: System chat: Incorrect argument for command
6566:gamerule doDaylightCycle false<--[HERE]
6632:[18:24:38 INFO]: System chat: Incorrect argument for command
6633:gamerule doDaylightCycle false<--[HERE]
6703:[18:24:44 INFO]: System chat: Incorrect argument for command
6704:gamerule doDaylightCycle false<--[HERE]
6793:[18:24:53 INFO]: System chat: Incorrect argument for command
6794:gamerule doDaylightCycle false<--[HERE]
6850:[18:24:58 INFO]: System chat: Incorrect argument for command
6851:gamerule doDaylightCycle false<--[HERE]
6918:[18:25:04 INFO]: System chat: Incorrect argument for command
6919:gamerule doDaylightCycle false<--[HERE]
7020:[18:25:13 INFO]: System chat: Incorrect argument for command
7021:gamerule doDaylightCycle false<--[HERE]
7094:[18:25:19 INFO]: System chat: Incorrect argument for command
7095:gamerule doDaylightCycle false<--[HERE]
7161:[18:25:25 INFO]: System chat: Incorrect argument for command
7162:gamerule doDaylightCycle false<--[HERE]
8833:[18:28:03 INFO]: System chat: Incorrect argument for command
8834:gamerule doDaylightCycle false<--[HERE]
9173:[18:28:35 INFO]: System chat: Incorrect argument for command
9174:gamerule doDaylightCycle false<--[HERE]
9514:[18:29:07 INFO]: System chat: Incorrect argument for command
9515:gamerule doDaylightCycle false<--[HERE]
9856:[18:29:38 INFO]: System chat: Incorrect argument for command
9857:gamerule doDaylightCycle false<--[HERE]
10198:[18:30:10 INFO]: System chat: Incorrect argument for command
10199:gamerule doDaylightCycle false<--[HERE]
10536:[18:30:42 INFO]: System chat: Incorrect argument for command
10537:gamerule doDaylightCycle false<--[HERE]
10874:[18:31:13 INFO]: System chat: Incorrect argument for command
10875:gamerule doDaylightCycle false<--[HERE]
11214:[18:31:45 INFO]: System chat: Incorrect argument for command
11215:gamerule doDaylightCycle false<--[HERE]
11558:[18:32:17 INFO]: System chat: Incorrect argument for command
11559:gamerule doDaylightCycle false<--[HERE]
11902:[18:32:48 INFO]: System chat: Incorrect argument for command
11903:gamerule doDaylightCycle false<--[HERE]
12241:[18:33:20 INFO]: System chat: Incorrect argument for command
12242:gamerule doDaylightCycle false<--[HERE]
12589:[18:33:52 INFO]: System chat: Incorrect argument for command
12590:gamerule doDaylightCycle false<--[HERE]
12928:[18:34:24 INFO]: System chat: Incorrect argument for command
12929:gamerule doDaylightCycle false<--[HERE]
--- bare-gamerule probe (v6.25.7 pre-registered; reply or new error echo, the known doDaylightCycle failure filtered out) ---
gamerule<--[HERE]
```
