# parity session time audit (the §7b console channel)

- v6.25.5: boot markers, pins, gamerules and joins from the parity session server.log (tail 40). The #332 night-matrix class is self-diagnosing from here on.

```
4040:[17:54:37 INFO]: System chat: Incorrect argument for command
4041:gamerule doDaylightCycle false<--[HERE]
4108:[17:54:43 INFO]: System chat: Incorrect argument for command
4109:gamerule doDaylightCycle false<--[HERE]
4174:[17:54:49 INFO]: System chat: Incorrect argument for command
4175:gamerule doDaylightCycle false<--[HERE]
4241:[17:54:55 INFO]: System chat: Incorrect argument for command
4242:gamerule doDaylightCycle false<--[HERE]
4311:[17:55:01 INFO]: System chat: Incorrect argument for command
4312:gamerule doDaylightCycle false<--[HERE]
4377:[17:55:07 INFO]: System chat: Incorrect argument for command
4378:gamerule doDaylightCycle false<--[HERE]
4445:[17:55:13 INFO]: System chat: Incorrect argument for command
4446:gamerule doDaylightCycle false<--[HERE]
4511:[17:55:19 INFO]: System chat: Incorrect argument for command
4512:gamerule doDaylightCycle false<--[HERE]
4579:[17:55:25 INFO]: System chat: Incorrect argument for command
4580:gamerule doDaylightCycle false<--[HERE]
4649:[17:55:31 INFO]: System chat: Incorrect argument for command
4650:gamerule doDaylightCycle false<--[HERE]
4715:[17:55:37 INFO]: System chat: Incorrect argument for command
4716:gamerule doDaylightCycle false<--[HERE]
4782:[17:55:43 INFO]: System chat: Incorrect argument for command
4783:gamerule doDaylightCycle false<--[HERE]
4848:[17:55:49 INFO]: System chat: Incorrect argument for command
4849:gamerule doDaylightCycle false<--[HERE]
4916:[17:55:55 INFO]: System chat: Incorrect argument for command
4917:gamerule doDaylightCycle false<--[HERE]
4986:[17:56:01 INFO]: System chat: Incorrect argument for command
4987:gamerule doDaylightCycle false<--[HERE]
5052:[17:56:07 INFO]: System chat: Incorrect argument for command
5053:gamerule doDaylightCycle false<--[HERE]
5120:[17:56:13 INFO]: System chat: Incorrect argument for command
5121:gamerule doDaylightCycle false<--[HERE]
5186:[17:56:19 INFO]: System chat: Incorrect argument for command
5187:gamerule doDaylightCycle false<--[HERE]
5253:[17:56:25 INFO]: System chat: Incorrect argument for command
5254:gamerule doDaylightCycle false<--[HERE]
5319:[17:56:31 INFO]: System chat: Incorrect argument for command
5320:gamerule doDaylightCycle false<--[HERE]
5389:[17:56:37 INFO]: System chat: Incorrect argument for command
5390:gamerule doDaylightCycle false<--[HERE]
5457:[17:56:43 INFO]: System chat: Incorrect argument for command
5458:gamerule doDaylightCycle false<--[HERE]
5522:[17:56:49 INFO]: System chat: Incorrect argument for command
5523:gamerule doDaylightCycle false<--[HERE]
5590:[17:56:55 INFO]: System chat: Incorrect argument for command
5591:gamerule doDaylightCycle false<--[HERE]
5656:[17:57:01 INFO]: System chat: Incorrect argument for command
5657:gamerule doDaylightCycle false<--[HERE]
5726:[17:57:07 INFO]: System chat: Incorrect argument for command
5727:gamerule doDaylightCycle false<--[HERE]
5794:[17:57:13 INFO]: System chat: Incorrect argument for command
5795:gamerule doDaylightCycle false<--[HERE]
5860:[17:57:19 INFO]: System chat: Incorrect argument for command
5861:gamerule doDaylightCycle false<--[HERE]
5928:[17:57:25 INFO]: System chat: Incorrect argument for command
5929:gamerule doDaylightCycle false<--[HERE]
5993:[17:57:31 INFO]: System chat: Incorrect argument for command
5994:gamerule doDaylightCycle false<--[HERE]
6063:[17:57:37 INFO]: System chat: Incorrect argument for command
6064:gamerule doDaylightCycle false<--[HERE]
6131:[17:57:43 INFO]: System chat: Incorrect argument for command
6132:gamerule doDaylightCycle false<--[HERE]
6197:[17:57:49 INFO]: System chat: Incorrect argument for command
6198:gamerule doDaylightCycle false<--[HERE]
6265:[17:57:55 INFO]: System chat: Incorrect argument for command
6266:gamerule doDaylightCycle false<--[HERE]
6331:[17:58:01 INFO]: System chat: Incorrect argument for command
6332:gamerule doDaylightCycle false<--[HERE]
6402:[17:58:07 INFO]: System chat: Incorrect argument for command
6403:gamerule doDaylightCycle false<--[HERE]
8072:[18:00:43 INFO]: System chat: Incorrect argument for command
8073:gamerule doDaylightCycle false<--[HERE]
8411:[18:01:15 INFO]: System chat: Incorrect argument for command
8412:gamerule doDaylightCycle false<--[HERE]
8751:[18:01:47 INFO]: System chat: Incorrect argument for command
8752:gamerule doDaylightCycle false<--[HERE]
9093:[18:02:18 INFO]: System chat: Incorrect argument for command
9094:gamerule doDaylightCycle false<--[HERE]
9430:[18:02:50 INFO]: System chat: Incorrect argument for command
9431:gamerule doDaylightCycle false<--[HERE]
9769:[18:03:22 INFO]: System chat: Incorrect argument for command
9770:gamerule doDaylightCycle false<--[HERE]
10113:[18:03:53 INFO]: System chat: Incorrect argument for command
10114:gamerule doDaylightCycle false<--[HERE]
10452:[18:04:25 INFO]: System chat: Incorrect argument for command
10453:gamerule doDaylightCycle false<--[HERE]
10794:[18:04:57 INFO]: System chat: Incorrect argument for command
10795:gamerule doDaylightCycle false<--[HERE]
11139:[18:05:28 INFO]: System chat: Incorrect argument for command
11140:gamerule doDaylightCycle false<--[HERE]
11479:[18:06:00 INFO]: System chat: Incorrect argument for command
11480:gamerule doDaylightCycle false<--[HERE]
11817:[18:06:32 INFO]: System chat: Incorrect argument for command
11818:gamerule doDaylightCycle false<--[HERE]
12158:[18:07:03 INFO]: System chat: Incorrect argument for command
12159:gamerule doDaylightCycle false<--[HERE]
12499:[18:07:35 INFO]: System chat: Incorrect argument for command
12500:gamerule doDaylightCycle false<--[HERE]
--- bare-gamerule probe (v6.25.7 pre-registered; reply or new error echo, the known doDaylightCycle failure filtered out) ---
gamerule<--[HERE]
```
