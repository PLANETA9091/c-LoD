# parity session time audit (the §7b console channel)

- v6.25.5: boot markers, pins, gamerules and joins from the parity session server.log (tail 40). The #332 night-matrix class is self-diagnosing from here on.

```
4220:[18:06:38 INFO]: System chat: Incorrect argument for command
4221:gamerule doDaylightCycle false<--[HERE]
4287:[18:06:44 INFO]: System chat: Incorrect argument for command
4288:gamerule doDaylightCycle false<--[HERE]
4358:[18:06:50 INFO]: System chat: Incorrect argument for command
4359:gamerule doDaylightCycle false<--[HERE]
4447:[18:06:58 INFO]: System chat: Incorrect argument for command
4448:gamerule doDaylightCycle false<--[HERE]
4548:[18:07:07 INFO]: System chat: Incorrect argument for command
4549:gamerule doDaylightCycle false<--[HERE]
4605:[18:07:13 INFO]: System chat: Incorrect argument for command
4606:gamerule doDaylightCycle false<--[HERE]
4674:[18:07:19 INFO]: System chat: Incorrect argument for command
4675:gamerule doDaylightCycle false<--[HERE]
4775:[18:07:28 INFO]: System chat: Incorrect argument for command
4776:gamerule doDaylightCycle false<--[HERE]
4842:[18:07:34 INFO]: System chat: Incorrect argument for command
4843:gamerule doDaylightCycle false<--[HERE]
4913:[18:07:40 INFO]: System chat: Incorrect argument for command
4914:gamerule doDaylightCycle false<--[HERE]
5002:[18:07:48 INFO]: System chat: Incorrect argument for command
5003:gamerule doDaylightCycle false<--[HERE]
5069:[18:07:54 INFO]: System chat: Incorrect argument for command
5070:gamerule doDaylightCycle false<--[HERE]
5142:[18:08:00 INFO]: System chat: Incorrect argument for command
5143:gamerule doDaylightCycle false<--[HERE]
5231:[18:08:09 INFO]: System chat: Incorrect argument for command
5232:gamerule doDaylightCycle false<--[HERE]
5333:[18:08:18 INFO]: System chat: Incorrect argument for command
5334:gamerule doDaylightCycle false<--[HERE]
5400:[18:08:24 INFO]: System chat: Incorrect argument for command
5401:gamerule doDaylightCycle false<--[HERE]
5470:[18:08:30 INFO]: System chat: Incorrect argument for command
5471:gamerule doDaylightCycle false<--[HERE]
5560:[18:08:38 INFO]: System chat: Incorrect argument for command
5561:gamerule doDaylightCycle false<--[HERE]
5627:[18:08:44 INFO]: System chat: Incorrect argument for command
5628:gamerule doDaylightCycle false<--[HERE]
5700:[18:08:50 INFO]: System chat: Incorrect argument for command
5701:gamerule doDaylightCycle false<--[HERE]
5789:[18:08:58 INFO]: System chat: Incorrect argument for command
5790:gamerule doDaylightCycle false<--[HERE]
5846:[18:09:04 INFO]: System chat: Incorrect argument for command
5847:gamerule doDaylightCycle false<--[HERE]
5917:[18:09:10 INFO]: System chat: Incorrect argument for command
5918:gamerule doDaylightCycle false<--[HERE]
6006:[18:09:18 INFO]: System chat: Incorrect argument for command
6007:gamerule doDaylightCycle false<--[HERE]
6073:[18:09:24 INFO]: System chat: Incorrect argument for command
6074:gamerule doDaylightCycle false<--[HERE]
6144:[18:09:30 INFO]: System chat: Incorrect argument for command
6145:gamerule doDaylightCycle false<--[HERE]
6233:[18:09:38 INFO]: System chat: Incorrect argument for command
6234:gamerule doDaylightCycle false<--[HERE]
6300:[18:09:44 INFO]: System chat: Incorrect argument for command
6301:gamerule doDaylightCycle false<--[HERE]
6373:[18:09:50 INFO]: System chat: Incorrect argument for command
6374:gamerule doDaylightCycle false<--[HERE]
6462:[18:09:59 INFO]: System chat: Incorrect argument for command
6463:gamerule doDaylightCycle false<--[HERE]
6529:[18:10:05 INFO]: System chat: Incorrect argument for command
6530:gamerule doDaylightCycle false<--[HERE]
6600:[18:10:11 INFO]: System chat: Incorrect argument for command
6601:gamerule doDaylightCycle false<--[HERE]
6679:[18:10:18 INFO]: System chat: Incorrect argument for command
6680:gamerule doDaylightCycle false<--[HERE]
6747:[18:10:24 INFO]: System chat: Incorrect argument for command
6748:gamerule doDaylightCycle false<--[HERE]
6817:[18:10:30 INFO]: System chat: Incorrect argument for command
6818:gamerule doDaylightCycle false<--[HERE]
6907:[18:10:38 INFO]: System chat: Incorrect argument for command
6908:gamerule doDaylightCycle false<--[HERE]
6973:[18:10:44 INFO]: System chat: Incorrect argument for command
6974:gamerule doDaylightCycle false<--[HERE]
8646:[18:13:21 INFO]: System chat: Incorrect argument for command
8647:gamerule doDaylightCycle false<--[HERE]
8987:[18:13:53 INFO]: System chat: Incorrect argument for command
8988:gamerule doDaylightCycle false<--[HERE]
9326:[18:14:24 INFO]: System chat: Incorrect argument for command
9327:gamerule doDaylightCycle false<--[HERE]
9672:[18:14:56 INFO]: System chat: Incorrect argument for command
9673:gamerule doDaylightCycle false<--[HERE]
10012:[18:15:28 INFO]: System chat: Incorrect argument for command
10013:gamerule doDaylightCycle false<--[HERE]
10350:[18:16:00 INFO]: System chat: Incorrect argument for command
10351:gamerule doDaylightCycle false<--[HERE]
10690:[18:16:31 INFO]: System chat: Incorrect argument for command
10691:gamerule doDaylightCycle false<--[HERE]
11028:[18:17:03 INFO]: System chat: Incorrect argument for command
11029:gamerule doDaylightCycle false<--[HERE]
11369:[18:17:34 INFO]: System chat: Incorrect argument for command
11370:gamerule doDaylightCycle false<--[HERE]
11715:[18:18:06 INFO]: System chat: Incorrect argument for command
11716:gamerule doDaylightCycle false<--[HERE]
12053:[18:18:38 INFO]: System chat: Incorrect argument for command
12054:gamerule doDaylightCycle false<--[HERE]
12393:[18:19:10 INFO]: System chat: Incorrect argument for command
12394:gamerule doDaylightCycle false<--[HERE]
12732:[18:19:41 INFO]: System chat: Incorrect argument for command
12733:gamerule doDaylightCycle false<--[HERE]
--- bare-gamerule probe (v6.25.7 pre-registered; reply or new error echo, the known doDaylightCycle failure filtered out) ---
gamerule<--[HERE]
```
