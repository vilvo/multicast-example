# Example of UDP multicast receiver witn libpnet

The example receives local multicast `239.12.255.254` to network interface `eth0`.

Using libpnet, based on packetdump-example, the received ethernet frame is mapped to IPv4 and finally to UDP packet.

`handle_udp_packet()` allows processing of the multicast packet header and payload.
