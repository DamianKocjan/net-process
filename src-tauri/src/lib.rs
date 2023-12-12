use std::collections::HashMap;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NetworkData {
    received: u64,
    sent: u64,
}

#[derive(Debug, Serialize)]
pub struct InterfaceStats {
    bytes: NetworkData,
    unicast_packets: NetworkData,
    non_unicast_packets: NetworkData,
    discards: NetworkData,
    errors: NetworkData,
    unknown_protocols: u64,
}

#[derive(Debug, Serialize)]
pub struct IpV4Statistics {
    packets_received: u64,
    received_header_errors: u64,
    received_address_errors: u64,
    datagrams_forwarded: u64,
    unknown_protocols_received: u64,
    received_packets_discarded: u64,
    received_packets_delivered: u64,
    output_requests: u64,
    routing_discards: u64,
    discarded_output_packets: u64,
    output_packet_no_route: u64,
    reassembly_required: u64,
    reassembly_successful: u64,
    reassembly_failures: u64,
    datagrams_successfully_fragmented: u64,
    datagrams_failing_fragmentation: u64,
    fragments_created: u64,
}

#[derive(Debug, Serialize)]
pub struct IpV6Statistics {
    packets_received: u64,
    received_header_errors: u64,
    received_address_errors: u64,
    datagrams_forwarded: u64,
    unknown_protocols_received: u64,
    received_packets_discarded: u64,
    received_packets_delivered: u64,
    output_requests: u64,
    routing_discards: u64,
    discarded_output_packets: u64,
    output_packet_no_route: u64,
    reassembly_required: u64,
    reassembly_successful: u64,
    reassembly_failures: u64,
    datagrams_successfully_fragmented: u64,
    datagrams_failing_fragmentation: u64,
    fragments_created: u64,
}

#[derive(Debug, Serialize)]
pub struct IcmpV4Statistics {
    messages: NetworkData,
    errors: NetworkData,
    destination_unreachable: NetworkData,
    time_exceeded: NetworkData,
    parameter_problems: NetworkData,
    source_quenches: NetworkData,
    redirects: NetworkData,
    echo_replies: NetworkData,
    echos: NetworkData,
    timestamps: NetworkData,
    timestamp_replies: NetworkData,
    address_masks: NetworkData,
    address_mask_replies: NetworkData,
    router_solicitations: NetworkData,
    router_advertisements: NetworkData,
}

#[derive(Debug, Serialize)]
pub struct IcmpV6Statistics {
    messages: NetworkData,
    errors: NetworkData,
    destination_unreachable: NetworkData,
    packet_too_big: NetworkData,
    time_exceeded: NetworkData,
    parameter_problems: NetworkData,
    echos: NetworkData,
    echo_replies: NetworkData,
    mld_queries: NetworkData,
    mld_reports: NetworkData,
    mld_dones: NetworkData,
    router_solicitations: NetworkData,
    router_advertisements: NetworkData,
    neighbor_solicitations: NetworkData,
    neighbor_advertisements: NetworkData,
    redirects: NetworkData,
    router_renumberings: NetworkData,
}

#[derive(Debug, Serialize)]
pub struct TcpV4Statistics {
    active_opens: u64,
    passive_opens: u64,
    failed_connection_attempts: u64,
    reset_connections: u64,
    current_connections: u64,
    segments_received: u64,
    segments_sent: u64,
    segments_retransmitted: u64,
}

#[derive(Debug, Serialize)]
pub struct TcpV6Statistics {
    active_opens: u64,
    passive_opens: u64,
    failed_connection_attempts: u64,
    reset_connections: u64,
    current_connections: u64,
    segments_received: u64,
    segments_sent: u64,
    segments_retransmitted: u64,
}

#[derive(Debug, Serialize)]
pub struct UdpV4Statistics {
    datagrams_received: u64,
    no_ports: u64,
    receive_errors: u64,
    datagrams_sent: u64,
}

#[derive(Debug, Serialize)]
pub struct UdpV6Statistics {
    datagrams_received: u64,
    no_ports: u64,
    receive_errors: u64,
    datagrams_sent: u64,
}

#[derive(Debug, Serialize)]
pub struct NetworkStatistics {
    interface: InterfaceStats,
    ipv4: IpV4Statistics,
    ipv6: IpV6Statistics,
    icmpv4: IcmpV4Statistics,
    icmpv6: IcmpV6Statistics,
    tcp_ipv4: TcpV4Statistics,
    tcp_ipv6: TcpV6Statistics,
    udp_ipv4: UdpV4Statistics,
    udp_ipv6: UdpV6Statistics,
}

trait DisplayStats {
    fn display(&self);
}

impl DisplayStats for NetworkStatistics {
    fn display(&self) {
        self.interface.display();
        self.ipv4.display();
        self.ipv6.display();
        self.icmpv4.display();
        self.icmpv6.display();
        self.tcp_ipv4.display();
        self.tcp_ipv6.display();
        self.udp_ipv4.display();
        self.udp_ipv6.display();
    }
}

impl DisplayStats for InterfaceStats {
    fn display(&self) {
        println!("Interface Statistics");
        println!("\tReceived: {:?}", self.bytes);
        println!("\tSent: {:?}", self.bytes);
        println!("\tUnicast packets: {:?}", self.unicast_packets);
        println!(
            "\tNon-unicast packets: {}",
            self.non_unicast_packets.received
        );
        println!("\tDiscards: {:?}", self.discards);
        println!("\tErrors: {:?}", self.errors);
        println!("\tUnknown protocols: {}", self.unknown_protocols);
    }
}

impl DisplayStats for IpV4Statistics {
    fn display(&self) {
        println!("IPv4 Statistics");
        println!("\tPackets received: {}", self.packets_received);
        println!("\tReceived header errors: {}", self.received_header_errors);
        println!(
            "\tReceived address errors: {}",
            self.received_address_errors
        );
        println!("\tDatagrams forwarded: {}", self.datagrams_forwarded);
        println!(
            "\tUnknown protocols received: {}",
            self.unknown_protocols_received
        );
        println!(
            "\tReceived packets discarded: {}",
            self.received_packets_discarded
        );
        println!(
            "\tReceived packets delivered: {}",
            self.received_packets_delivered
        );
        println!("\tOutput requests: {}", self.output_requests);
        println!("\tRouting discards: {}", self.routing_discards);
        println!(
            "\tDiscarded output packets: {}",
            self.discarded_output_packets
        );
        println!("\tOutput packet no route: {}", self.output_packet_no_route);
        println!("\tReassembly required: {}", self.reassembly_required);
        println!("\tReassembly successful: {}", self.reassembly_successful);
        println!("\tReassembly failures: {}", self.reassembly_failures);
        println!(
            "\tDatagrams successfully fragmented: {}",
            self.datagrams_successfully_fragmented
        );
        println!(
            "\tDatagrams failing fragmentation: {}",
            self.datagrams_failing_fragmentation
        );
        println!("\tFragments created: {}", self.fragments_created);
    }
}

impl DisplayStats for IpV6Statistics {
    fn display(&self) {
        println!("IPv6 Statistics");
        println!("\tPackets received: {}", self.packets_received);
        println!("\tReceived header errors: {}", self.received_header_errors);
        println!(
            "\tReceived address errors: {}",
            self.received_address_errors
        );
        println!("\tDatagrams forwarded: {}", self.datagrams_forwarded);
        println!(
            "\tUnknown protocols received: {}",
            self.unknown_protocols_received
        );
        println!(
            "\tReceived packets discarded: {}",
            self.received_packets_discarded
        );
        println!(
            "\tReceived packets delivered: {}",
            self.received_packets_delivered
        );
        println!("\tOutput requests: {}", self.output_requests);
        println!("\tRouting discards: {}", self.routing_discards);
        println!(
            "\tDiscarded output packets: {}",
            self.discarded_output_packets
        );
        println!("\tOutput packet no route: {}", self.output_packet_no_route);
        println!("\tReassembly required: {}", self.reassembly_required);
        println!("\tReassembly successful: {}", self.reassembly_successful);
        println!("\tReassembly failures: {}", self.reassembly_failures);
        println!(
            "\tDatagrams successfully fragmented: {}",
            self.datagrams_successfully_fragmented
        );
        println!(
            "\tDatagrams failing fragmentation: {}",
            self.datagrams_failing_fragmentation
        );
        println!("\tFragments created: {}", self.fragments_created);
    }
}

impl DisplayStats for IcmpV4Statistics {
    fn display(&self) {
        println!("ICMPv4 Statistics");
        println!("\tMessages: {:?}", self.messages);
        println!("\tErrors: {:?}", self.errors);
        println!(
            "\tDestination unreachable: {:?}",
            self.destination_unreachable
        );
        println!("\tTime exceeded: {:?}", self.time_exceeded);
        println!("\tParameter problems: {:?}", self.parameter_problems);
        println!("\tSource quenches: {:?}", self.source_quenches);
        println!("\tRedirects: {:?}", self.redirects);
        println!("\tEcho replies: {:?}", self.echo_replies);
        println!("\tEchos: {:?}", self.echos);
        println!("\tTimestamps: {:?}", self.timestamps);
        println!("\tTimestamp replies: {:?}", self.timestamp_replies);
        println!("\tAddress masks: {:?}", self.address_masks);
        println!("\tAddress mask replies: {:?}", self.address_mask_replies);
        println!("\tRouter solicitations: {:?}", self.router_solicitations);
        println!("\tRouter advertisements: {:?}", self.router_advertisements);
    }
}

impl DisplayStats for IcmpV6Statistics {
    fn display(&self) {
        println!("ICMPv6 Statistics");
        println!("\tMessages: {:?}", self.messages);
        println!("\tErrors: {:?}", self.errors);
        println!(
            "\tDestination unreachable: {:?}",
            self.destination_unreachable
        );
        println!("\tPacket too big: {:?}", self.packet_too_big);
        println!("\tTime exceeded: {:?}", self.time_exceeded);
        println!("\tParameter problems: {:?}", self.parameter_problems);
        println!("\tEchos: {:?}", self.echos);
        println!("\tEcho replies: {:?}", self.echo_replies);
        println!("\tMLD queries: {:?}", self.mld_queries);
        println!("\tMLD reports: {:?}", self.mld_reports);
        println!("\tMLD dones: {:?}", self.mld_dones);
        println!("\tRouter solicitations: {:?}", self.router_solicitations);
        println!("\tRouter advertisements: {:?}", self.router_advertisements);
        println!(
            "\tNeighbor solicitations: {:?}",
            self.neighbor_solicitations
        );
        println!(
            "\tNeighbor advertisements: {:?}",
            self.neighbor_advertisements
        );
        println!("\tRedirects: {:?}", self.redirects);
        println!("\tRouter renumberings: {:?}", self.router_renumberings);
    }
}

impl DisplayStats for TcpV4Statistics {
    fn display(&self) {
        println!("TCP Statistics for IPv4");
        println!("\tActive opens: {}", self.active_opens);
        println!("\tPassive opens: {}", self.passive_opens);
        println!(
            "\tFailed connection attempts: {}",
            self.failed_connection_attempts
        );
        println!("\tReset connections: {}", self.reset_connections);
        println!("\tCurrent connections: {}", self.current_connections);
        println!("\tSegments received: {}", self.segments_received);
        println!("\tSegments sent: {}", self.segments_sent);
        println!("\tSegments retransmitted: {}", self.segments_retransmitted);
    }
}

impl DisplayStats for TcpV6Statistics {
    fn display(&self) {
        println!("TCP Statistics for IPv6");
        println!("\tActive opens: {}", self.active_opens);
        println!("\tPassive opens: {}", self.passive_opens);
        println!(
            "\tFailed connection attempts: {}",
            self.failed_connection_attempts
        );
        println!("\tReset connections: {}", self.reset_connections);
        println!("\tCurrent connections: {}", self.current_connections);
        println!("\tSegments received: {}", self.segments_received);
        println!("\tSegments sent: {}", self.segments_sent);
        println!("\tSegments retransmitted: {}", self.segments_retransmitted);
    }
}

impl DisplayStats for UdpV4Statistics {
    fn display(&self) {
        println!("UDP Statistics for IPv4");
        println!("\tDatagrams received: {}", self.datagrams_received);
        println!("\tNo ports: {}", self.no_ports);
        println!("\tReceive errors: {}", self.receive_errors);
        println!("\tDatagrams sent: {}", self.datagrams_sent);
    }
}

impl DisplayStats for UdpV6Statistics {
    fn display(&self) {
        println!("UDP Statistics for IPv6");
        println!("\tDatagrams received: {}", self.datagrams_received);
        println!("\tNo ports: {}", self.no_ports);
        println!("\tReceive errors: {}", self.receive_errors);
        println!("\tDatagrams sent: {}", self.datagrams_sent);
    }
}

impl NetworkStatistics {
    fn new() -> Self {
        NetworkStatistics {
            interface: InterfaceStats {
                bytes: NetworkData {
                    received: 0,
                    sent: 0,
                },
                unicast_packets: NetworkData {
                    received: 0,
                    sent: 0,
                },
                non_unicast_packets: NetworkData {
                    received: 0,
                    sent: 0,
                },
                discards: NetworkData {
                    received: 0,
                    sent: 0,
                },
                errors: NetworkData {
                    received: 0,
                    sent: 0,
                },
                unknown_protocols: 0,
            },
            ipv4: IpV4Statistics {
                packets_received: 0,
                received_header_errors: 0,
                received_address_errors: 0,
                datagrams_forwarded: 0,
                unknown_protocols_received: 0,
                received_packets_discarded: 0,
                received_packets_delivered: 0,
                output_requests: 0,
                routing_discards: 0,
                discarded_output_packets: 0,
                output_packet_no_route: 0,
                reassembly_required: 0,
                reassembly_successful: 0,
                reassembly_failures: 0,
                datagrams_successfully_fragmented: 0,
                datagrams_failing_fragmentation: 0,
                fragments_created: 0,
            },
            ipv6: IpV6Statistics {
                packets_received: 0,
                received_header_errors: 0,
                received_address_errors: 0,
                datagrams_forwarded: 0,
                unknown_protocols_received: 0,
                received_packets_discarded: 0,
                received_packets_delivered: 0,
                output_requests: 0,
                routing_discards: 0,
                discarded_output_packets: 0,
                output_packet_no_route: 0,
                reassembly_required: 0,
                reassembly_successful: 0,
                reassembly_failures: 0,
                datagrams_successfully_fragmented: 0,
                datagrams_failing_fragmentation: 0,
                fragments_created: 0,
            },
            icmpv4: IcmpV4Statistics {
                messages: NetworkData {
                    received: 0,
                    sent: 0,
                },
                errors: NetworkData {
                    received: 0,
                    sent: 0,
                },
                destination_unreachable: NetworkData {
                    received: 0,
                    sent: 0,
                },
                time_exceeded: NetworkData {
                    received: 0,
                    sent: 0,
                },
                parameter_problems: NetworkData {
                    received: 0,
                    sent: 0,
                },
                source_quenches: NetworkData {
                    received: 0,
                    sent: 0,
                },
                redirects: NetworkData {
                    received: 0,
                    sent: 0,
                },
                echo_replies: NetworkData {
                    received: 0,
                    sent: 0,
                },
                echos: NetworkData {
                    received: 0,
                    sent: 0,
                },
                timestamps: NetworkData {
                    received: 0,
                    sent: 0,
                },
                timestamp_replies: NetworkData {
                    received: 0,
                    sent: 0,
                },
                address_masks: NetworkData {
                    received: 0,
                    sent: 0,
                },
                address_mask_replies: NetworkData {
                    received: 0,
                    sent: 0,
                },
                router_solicitations: NetworkData {
                    received: 0,
                    sent: 0,
                },
                router_advertisements: NetworkData {
                    received: 0,
                    sent: 0,
                },
            },
            icmpv6: IcmpV6Statistics {
                messages: NetworkData {
                    received: 0,
                    sent: 0,
                },
                errors: NetworkData {
                    received: 0,
                    sent: 0,
                },
                destination_unreachable: NetworkData {
                    received: 0,
                    sent: 0,
                },
                packet_too_big: NetworkData {
                    received: 0,
                    sent: 0,
                },
                time_exceeded: NetworkData {
                    received: 0,
                    sent: 0,
                },
                parameter_problems: NetworkData {
                    received: 0,
                    sent: 0,
                },
                echos: NetworkData {
                    received: 0,
                    sent: 0,
                },
                echo_replies: NetworkData {
                    received: 0,
                    sent: 0,
                },
                mld_queries: NetworkData {
                    received: 0,
                    sent: 0,
                },
                mld_reports: NetworkData {
                    received: 0,
                    sent: 0,
                },
                mld_dones: NetworkData {
                    received: 0,
                    sent: 0,
                },
                router_solicitations: NetworkData {
                    received: 0,
                    sent: 0,
                },
                router_advertisements: NetworkData {
                    received: 0,
                    sent: 0,
                },
                neighbor_solicitations: NetworkData {
                    received: 0,
                    sent: 0,
                },
                neighbor_advertisements: NetworkData {
                    received: 0,
                    sent: 0,
                },
                redirects: NetworkData {
                    received: 0,
                    sent: 0,
                },
                router_renumberings: NetworkData {
                    received: 0,
                    sent: 0,
                },
            },
            tcp_ipv4: TcpV4Statistics {
                active_opens: 0,
                passive_opens: 0,
                failed_connection_attempts: 0,
                reset_connections: 0,
                current_connections: 0,
                segments_received: 0,
                segments_sent: 0,
                segments_retransmitted: 0,
            },
            tcp_ipv6: TcpV6Statistics {
                active_opens: 0,
                passive_opens: 0,
                failed_connection_attempts: 0,
                reset_connections: 0,
                current_connections: 0,
                segments_received: 0,
                segments_sent: 0,
                segments_retransmitted: 0,
            },
            udp_ipv4: UdpV4Statistics {
                datagrams_received: 0,
                no_ports: 0,
                receive_errors: 0,
                datagrams_sent: 0,
            },
            udp_ipv6: UdpV6Statistics {
                datagrams_received: 0,
                no_ports: 0,
                receive_errors: 0,
                datagrams_sent: 0,
            },
        }
    }
}

pub fn parse_stats(input: &str) -> NetworkStatistics {
    let mut stats = NetworkStatistics::new();

    let mut current_section = "";
    let lines = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty());

    for line in lines {
        let tokens: Vec<&str> = line.split_whitespace().collect();

        match tokens.as_slice() {
            ["Bytes", received, sent] => {
                stats.interface.bytes.received = received.parse().unwrap();
                stats.interface.bytes.sent = sent.parse().unwrap();
            }
            ["Unicast", "packets", received, sent] => {
                stats.interface.unicast_packets.received = received.parse().unwrap();
                stats.interface.unicast_packets.sent = sent.parse().unwrap();
            }
            ["Non-unicast", "packets", received, sent] => {
                stats.interface.non_unicast_packets.received = received.parse().unwrap();
                stats.interface.non_unicast_packets.sent = sent.parse().unwrap();
            }
            ["Discards", received, sent] => {
                stats.interface.discards.received = received.parse().unwrap();
                stats.interface.discards.sent = sent.parse().unwrap();
            }
            ["Errors", received, sent] => match current_section {
                "ICMPv4" => {
                    stats.icmpv4.errors.received = received.parse().unwrap();
                    stats.icmpv4.errors.sent = sent.parse().unwrap();
                }
                "ICMPv6" => {
                    stats.icmpv6.errors.received = received.parse().unwrap();
                    stats.icmpv6.errors.sent = sent.parse().unwrap();
                }
                _ => {
                    stats.interface.errors.received = received.parse().unwrap();
                    stats.interface.errors.sent = sent.parse().unwrap();
                }
            },
            ["Unknown", "protocols", unknown_protocols] => {
                stats.interface.unknown_protocols = unknown_protocols.parse().unwrap();
            }
            ["Packets", "Received", "=", packets_received] => match current_section {
                "IPv4" => stats.ipv4.packets_received = packets_received.parse().unwrap(),
                "IPv6" => stats.ipv6.packets_received = packets_received.parse().unwrap(),
                _ => (),
            },
            ["Received", "Header", "Errors", "=", received_header_errors] => {
                match current_section {
                    "IPv4" => {
                        stats.ipv4.received_header_errors = received_header_errors.parse().unwrap()
                    }
                    "IPv6" => {
                        stats.ipv6.received_header_errors = received_header_errors.parse().unwrap()
                    }
                    _ => (),
                }
            }
            ["Received", "Address", "Errors", "=", received_address_errors] => {
                match current_section {
                    "IPv4" => {
                        stats.ipv4.received_address_errors =
                            received_address_errors.parse().unwrap()
                    }
                    "IPv6" => {
                        stats.ipv6.received_address_errors =
                            received_address_errors.parse().unwrap()
                    }
                    _ => (),
                }
            }
            ["Datagrams", "Forwarded", "=", datagrams_forwarded] => match current_section {
                "IPv4" => stats.ipv4.datagrams_forwarded = datagrams_forwarded.parse().unwrap(),
                "IPv6" => stats.ipv6.datagrams_forwarded = datagrams_forwarded.parse().unwrap(),
                _ => (),
            },
            ["Unknown", "Protocols", "Received", "=", unknown_protocols_received] => {
                match current_section {
                    "IPv4" => {
                        stats.ipv4.unknown_protocols_received =
                            unknown_protocols_received.parse().unwrap()
                    }
                    "IPv6" => {
                        stats.ipv6.unknown_protocols_received =
                            unknown_protocols_received.parse().unwrap()
                    }
                    _ => (),
                }
            }
            ["Received", "Packets", "Discarded", "=", received_packets_discarded] => {
                match current_section {
                    "IPv4" => {
                        stats.ipv4.received_packets_discarded =
                            received_packets_discarded.parse().unwrap()
                    }
                    "IPv6" => {
                        stats.ipv6.received_packets_discarded =
                            received_packets_discarded.parse().unwrap()
                    }
                    _ => (),
                }
            }
            ["Received", "Packets", "Delivered", "=", received_packets_delivered] => {
                match current_section {
                    "IPv4" => {
                        stats.ipv4.received_packets_delivered =
                            received_packets_delivered.parse().unwrap()
                    }
                    "IPv6" => {
                        stats.ipv6.received_packets_delivered =
                            received_packets_delivered.parse().unwrap()
                    }
                    _ => (),
                }
            }
            ["Output", "Requests", "=", output_requests] => match current_section {
                "IPv4" => stats.ipv4.output_requests = output_requests.parse().unwrap(),
                "IPv6" => stats.ipv6.output_requests = output_requests.parse().unwrap(),
                _ => (),
            },
            ["Routing", "Discards", "=", routing_discards] => match current_section {
                "IPv4" => stats.ipv4.routing_discards = routing_discards.parse().unwrap(),
                "IPv6" => stats.ipv6.routing_discards = routing_discards.parse().unwrap(),
                _ => (),
            },
            ["Discarded", "Output", "Packets", "=", discarded_output_packets] => {
                match current_section {
                    "IPv4" => {
                        stats.ipv4.discarded_output_packets =
                            discarded_output_packets.parse().unwrap()
                    }
                    "IPv6" => {
                        stats.ipv6.discarded_output_packets =
                            discarded_output_packets.parse().unwrap()
                    }
                    _ => (),
                }
            }
            ["Output", "Packet", "No", "Route", "=", output_packet_no_route] => {
                match current_section {
                    "IPv4" => {
                        stats.ipv4.output_packet_no_route = output_packet_no_route.parse().unwrap()
                    }
                    "IPv6" => {
                        stats.ipv6.output_packet_no_route = output_packet_no_route.parse().unwrap()
                    }
                    _ => (),
                }
            }
            ["Reassembly", "Required", "=", reassembly_required] => match current_section {
                "IPv4" => stats.ipv4.reassembly_required = reassembly_required.parse().unwrap(),
                "IPv6" => stats.ipv6.reassembly_required = reassembly_required.parse().unwrap(),
                _ => (),
            },
            ["Reassembly", "Successful", "=", reassembly_successful] => match current_section {
                "IPv4" => stats.ipv4.reassembly_successful = reassembly_successful.parse().unwrap(),
                "IPv6" => stats.ipv6.reassembly_successful = reassembly_successful.parse().unwrap(),
                _ => (),
            },
            ["Reassembly", "Failures", "=", reassembly_failures] => match current_section {
                "IPv4" => stats.ipv4.reassembly_failures = reassembly_failures.parse().unwrap(),
                "IPv6" => stats.ipv6.reassembly_failures = reassembly_failures.parse().unwrap(),
                _ => (),
            },
            ["Datagrams", "Successfully", "Fragmented", "=", datagrams_successfully_fragmented] => {
                match current_section {
                    "IPv4" => {
                        stats.ipv4.datagrams_successfully_fragmented =
                            datagrams_successfully_fragmented.parse().unwrap()
                    }
                    "IPv6" => {
                        stats.ipv6.datagrams_successfully_fragmented =
                            datagrams_successfully_fragmented.parse().unwrap()
                    }
                    _ => (),
                }
            }
            ["Datagrams", "Failing", "Fragmentation", "=", datagrams_failing_fragmentation] => {
                match current_section {
                    "IPv4" => {
                        stats.ipv4.datagrams_failing_fragmentation =
                            datagrams_failing_fragmentation.parse().unwrap()
                    }
                    "IPv6" => {
                        stats.ipv6.datagrams_failing_fragmentation =
                            datagrams_failing_fragmentation.parse().unwrap()
                    }
                    _ => (),
                }
            }
            ["Fragments", "Created", "=", fragments_created] => match current_section {
                "IPv4" => stats.ipv4.fragments_created = fragments_created.parse().unwrap(),
                "IPv6" => stats.ipv6.fragments_created = fragments_created.parse().unwrap(),
                _ => (),
            },
            ["Messages", messages_received, messages_sent] => match current_section {
                "ICMPv4" => {
                    stats.icmpv4.messages.received = messages_received.parse().unwrap();
                    stats.icmpv4.messages.sent = messages_sent.parse().unwrap();
                }
                "ICMPv6" => {
                    stats.icmpv6.messages.received = messages_received.parse().unwrap();
                    stats.icmpv6.messages.sent = messages_sent.parse().unwrap();
                }
                _ => (),
            },
            ["Destination", "Unreachable", received, sent] => match current_section {
                "ICMPv4" => {
                    stats.icmpv4.destination_unreachable.received = received.parse().unwrap();
                    stats.icmpv4.destination_unreachable.sent = sent.parse().unwrap();
                }
                "ICMPv6" => {
                    stats.icmpv6.destination_unreachable.received = received.parse().unwrap();
                    stats.icmpv6.destination_unreachable.sent = sent.parse().unwrap();
                }
                _ => (),
            },
            ["Time", "Exceeded", received, sent] => match current_section {
                "ICMPv4" => {
                    stats.icmpv4.time_exceeded.received = received.parse().unwrap();
                    stats.icmpv4.time_exceeded.sent = sent.parse().unwrap();
                }
                "ICMPv6" => {
                    stats.icmpv6.time_exceeded.received = received.parse().unwrap();
                    stats.icmpv6.time_exceeded.sent = sent.parse().unwrap();
                }
                _ => (),
            },
            ["Parameter", "Problems", received, sent] => match current_section {
                "ICMPv4" => {
                    stats.icmpv4.parameter_problems.received = received.parse().unwrap();
                    stats.icmpv4.parameter_problems.sent = sent.parse().unwrap();
                }
                "ICMPv6" => {
                    stats.icmpv6.parameter_problems.received = received.parse().unwrap();
                    stats.icmpv6.parameter_problems.sent = sent.parse().unwrap();
                }
                _ => (),
            },
            ["Source", "Quenches", received, sent] => match current_section {
                "ICMPv4" => {
                    stats.icmpv4.source_quenches.received = received.parse().unwrap();
                    stats.icmpv4.source_quenches.sent = sent.parse().unwrap();
                }
                _ => (),
            },
            ["Redirects", received, sent] => match current_section {
                "ICMPv4" => {
                    stats.icmpv4.redirects.received = received.parse().unwrap();
                    stats.icmpv4.redirects.sent = sent.parse().unwrap();
                }
                "ICMPv6" => {
                    stats.icmpv6.redirects.received = received.parse().unwrap();
                    stats.icmpv6.redirects.sent = sent.parse().unwrap();
                }
                _ => (),
            },
            ["Echo", "Replies", received, sent] => match current_section {
                "ICMPv4" => {
                    stats.icmpv4.echo_replies.received = received.parse().unwrap();
                    stats.icmpv4.echo_replies.sent = sent.parse().unwrap();
                }
                "ICMPv6" => {
                    stats.icmpv6.echo_replies.received = received.parse().unwrap();
                    stats.icmpv6.echo_replies.sent = sent.parse().unwrap();
                }
                _ => (),
            },
            ["Echos", received, sent] => match current_section {
                "ICMPv4" => {
                    stats.icmpv4.echos.received = received.parse().unwrap();
                    stats.icmpv4.echos.sent = sent.parse().unwrap();
                }
                "ICMPv6" => {
                    stats.icmpv6.echos.received = received.parse().unwrap();
                    stats.icmpv6.echos.sent = sent.parse().unwrap();
                }
                _ => (),
            },
            ["Timestamps", received, sent] => match current_section {
                "ICMPv4" => {
                    stats.icmpv4.timestamps.received = received.parse().unwrap();
                    stats.icmpv4.timestamps.sent = sent.parse().unwrap();
                }
                _ => (),
            },
            ["Timestamps", "Replies", received, sent] => match current_section {
                "ICMPv4" => {
                    stats.icmpv4.timestamps.received = received.parse().unwrap();
                    stats.icmpv4.timestamps.sent = sent.parse().unwrap();
                }
                _ => (),
            },
            ["Address", "Masks", received, sent] => match current_section {
                "ICMPv4" => {
                    stats.icmpv4.address_masks.received = received.parse().unwrap();
                    stats.icmpv4.address_masks.sent = sent.parse().unwrap();
                }
                _ => (),
            },
            ["Address", "Masks", "Replies", received, sent] => match current_section {
                "ICMPv4" => {
                    stats.icmpv4.address_mask_replies.received = received.parse().unwrap();
                    stats.icmpv4.address_mask_replies.sent = sent.parse().unwrap();
                }
                _ => (),
            },
            ["MLD", "Queries", received, sent] => match current_section {
                "ICMPv6" => {
                    stats.icmpv6.mld_queries.received = received.parse().unwrap();
                    stats.icmpv6.mld_queries.sent = sent.parse().unwrap();
                }
                _ => (),
            },
            ["MLD", "Reports", received, sent] => match current_section {
                "ICMPv6" => {
                    stats.icmpv6.mld_reports.received = received.parse().unwrap();
                    stats.icmpv6.mld_reports.sent = sent.parse().unwrap();
                }
                _ => (),
            },
            ["MLD", "Dones", received, sent] => match current_section {
                "ICMPv6" => {
                    stats.icmpv6.mld_dones.received = received.parse().unwrap();
                    stats.icmpv6.mld_dones.sent = sent.parse().unwrap();
                }
                _ => (),
            },
            ["Router", "Solicitations", received, sent] => match current_section {
                "ICMPv4" => {
                    stats.icmpv4.router_solicitations.received = received.parse().unwrap();
                    stats.icmpv4.router_solicitations.sent = sent.parse().unwrap();
                }
                "ICMPv6" => {
                    stats.icmpv6.router_solicitations.received = received.parse().unwrap();
                    stats.icmpv6.router_solicitations.sent = sent.parse().unwrap();
                }
                _ => (),
            },
            ["Router", "Advertisements", received, sent] => match current_section {
                "ICMPv4" => {
                    stats.icmpv4.router_advertisements.received = received.parse().unwrap();
                    stats.icmpv4.router_advertisements.sent = sent.parse().unwrap();
                }
                "ICMPv6" => {
                    stats.icmpv6.router_advertisements.received = received.parse().unwrap();
                    stats.icmpv6.router_advertisements.sent = sent.parse().unwrap();
                }
                _ => (),
            },
            ["Neighbor", "Solicitations", received, sent] => match current_section {
                "ICMPv6" => {
                    stats.icmpv6.neighbor_solicitations.received = received.parse().unwrap();
                    stats.icmpv6.neighbor_solicitations.sent = sent.parse().unwrap();
                }
                _ => (),
            },
            ["Neighbor", "Advertisements", received, sent] => match current_section {
                "ICMPv6" => {
                    stats.icmpv6.neighbor_advertisements.received = received.parse().unwrap();
                    stats.icmpv6.neighbor_advertisements.sent = sent.parse().unwrap();
                }
                _ => (),
            },
            ["Router", "Renumberings", received, sent] => match current_section {
                "ICMPv6" => {
                    stats.icmpv6.router_renumberings.received = received.parse().unwrap();
                    stats.icmpv6.router_renumberings.sent = sent.parse().unwrap();
                }
                _ => (),
            },
            ["Active", "Opens", "=", active_opens] => match current_section {
                "TCP Statistics for IPv4" => {
                    stats.tcp_ipv4.active_opens = active_opens.parse().unwrap()
                }
                "TCP Statistics for IPv6" => {
                    stats.tcp_ipv6.active_opens = active_opens.parse().unwrap()
                }
                _ => (),
            },
            ["Passive", "Opens", "=", passive_opens] => match current_section {
                "TCP Statistics for IPv4" => {
                    stats.tcp_ipv4.passive_opens = passive_opens.parse().unwrap()
                }
                "TCP Statistics for IPv6" => {
                    stats.tcp_ipv6.passive_opens = passive_opens.parse().unwrap()
                }
                _ => (),
            },
            ["Failed", "Connection", "Attempts", "=", failed_connection_attempts] => {
                match current_section {
                    "TCP Statistics for IPv4" => {
                        stats.tcp_ipv4.failed_connection_attempts =
                            failed_connection_attempts.parse().unwrap()
                    }
                    "TCP Statistics for IPv6" => {
                        stats.tcp_ipv6.failed_connection_attempts =
                            failed_connection_attempts.parse().unwrap()
                    }
                    _ => (),
                }
            }
            ["Reset", "Connections", "=", reset_connections] => match current_section {
                "TCP Statistics for IPv4" => {
                    stats.tcp_ipv4.reset_connections = reset_connections.parse().unwrap()
                }
                "TCP Statistics for IPv6" => {
                    stats.tcp_ipv6.reset_connections = reset_connections.parse().unwrap()
                }
                _ => (),
            },
            ["Current", "Connections", "=", current_connections] => match current_section {
                "TCP Statistics for IPv4" => {
                    stats.tcp_ipv4.current_connections = current_connections.parse().unwrap()
                }
                "TCP Statistics for IPv6" => {
                    stats.tcp_ipv6.current_connections = current_connections.parse().unwrap()
                }
                _ => (),
            },
            ["Segments", "Received", "=", segments_received] => match current_section {
                "TCP Statistics for IPv4" => {
                    stats.tcp_ipv4.segments_received = segments_received.parse().unwrap()
                }
                "TCP Statistics for IPv6" => {
                    stats.tcp_ipv6.segments_received = segments_received.parse().unwrap()
                }
                _ => (),
            },
            ["Segments", "Sent", "=", segments_sent] => match current_section {
                "TCP Statistics for IPv4" => {
                    stats.tcp_ipv4.segments_sent = segments_sent.parse().unwrap()
                }
                "TCP Statistics for IPv6" => {
                    stats.tcp_ipv6.segments_sent = segments_sent.parse().unwrap()
                }
                _ => (),
            },
            ["Segments", "Retransmitted", "=", segments_retransmitted] => match current_section {
                "TCP Statistics for IPv4" => {
                    stats.tcp_ipv4.segments_retransmitted = segments_retransmitted.parse().unwrap()
                }
                "TCP Statistics for IPv6" => {
                    stats.tcp_ipv6.segments_retransmitted = segments_retransmitted.parse().unwrap()
                }
                _ => (),
            },
            ["Datagrams", "Received", "=", datagrams_received] => match current_section {
                "UDP Statistics for IPv4" => {
                    stats.udp_ipv4.datagrams_received = datagrams_received.parse().unwrap()
                }
                "UDP Statistics for IPv6" => {
                    stats.udp_ipv6.datagrams_received = datagrams_received.parse().unwrap()
                }
                _ => (),
            },
            ["No", "Ports", "=", no_ports] => match current_section {
                "UDP Statistics for IPv4" => stats.udp_ipv4.no_ports = no_ports.parse().unwrap(),
                "UDP Statistics for IPv6" => stats.udp_ipv6.no_ports = no_ports.parse().unwrap(),
                _ => (),
            },
            ["Receive", "Errors", "=", receive_errors] => match current_section {
                "UDP Statistics for IPv4" => {
                    stats.udp_ipv4.receive_errors = receive_errors.parse().unwrap()
                }
                "UDP Statistics for IPv6" => {
                    stats.udp_ipv6.receive_errors = receive_errors.parse().unwrap()
                }
                _ => (),
            },
            ["Datagrams", "Sent", "=", datagrams_sent] => match current_section {
                "UDP Statistics for IPv4" => {
                    stats.udp_ipv4.datagrams_sent = datagrams_sent.parse().unwrap()
                }
                "UDP Statistics for IPv6" => {
                    stats.udp_ipv6.datagrams_sent = datagrams_sent.parse().unwrap()
                }
                _ => (),
            },
            _ => (),
        }

        if line.contains("IPv4 Statistics") {
            current_section = "IPv4";
        } else if line.contains("IPv6 Statistics") {
            current_section = "IPv6";
        } else if line.contains("ICMPv4 Statistics") {
            current_section = "ICMPv4";
        } else if line.contains("ICMPv6 Statistics") {
            current_section = "ICMPv6";
        } else if line.contains("TCP Statistics for IPv4") {
            current_section = "TCP Statistics for IPv4";
        } else if line.contains("TCP Statistics for IPv6") {
            current_section = "TCP Statistics for IPv6";
        } else if line.contains("UDP Statistics for IPv4") {
            current_section = "UDP Statistics for IPv4";
        } else if line.contains("UDP Statistics for IPv6") {
            current_section = "UDP Statistics for IPv6";
        }
    }

    stats
}

#[derive(Debug, Serialize)]
pub enum ConnectionType {
    Tcp,
    Udp,
}

#[derive(Debug, Serialize)]
pub enum ConnectionState {
    Established,
    SynSent,
    SynRecv,
    FinWait1,
    FinWait2,
    TimeWait,
    Close,
    CloseWait,
    LastAck,
    Listen,
    Closing,
}

#[derive(Debug, Serialize)]
pub struct Connection {
    pub connection_type: ConnectionType,
    pub local_address: String,
    pub foreign_address: String,
    pub state: ConnectionState,
    pub pid: String,
}

pub fn parse_connections(input: &str) -> Vec<Connection> {
    let mut connections = Vec::new();

    let lines = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .skip(2);

    for line in lines {
        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.len() < 4 {
            continue;
        }

        let connection_type = match tokens[0] {
            "TCP" => ConnectionType::Tcp,
            "UDP" => ConnectionType::Udp,
            _ => continue,
        };

        let local_address = tokens[1];
        let foreign_address = tokens[2];
        let state = match tokens[3] {
            "ESTABLISHED" => ConnectionState::Established,
            "SYN_SENT" => ConnectionState::SynSent,
            "SYN_RECV" => ConnectionState::SynRecv,
            "FIN_WAIT1" => ConnectionState::FinWait1,
            "FIN_WAIT2" => ConnectionState::FinWait2,
            "TIME_WAIT" => ConnectionState::TimeWait,
            "CLOSE" => ConnectionState::Close,
            "CLOSE_WAIT" => ConnectionState::CloseWait,
            "LAST_ACK" => ConnectionState::LastAck,
            "LISTEN" => ConnectionState::Listen,
            "CLOSING" => ConnectionState::Closing,
            _ => continue,
        };
        let pid = tokens[4];

        connections.push(Connection {
            connection_type,
            local_address: local_address.to_string(),
            foreign_address: foreign_address.to_string(),
            state,
            pid: pid.to_string(),
        });
    }

    connections
}

pub fn group_connections_by_pid(connections: Vec<Connection>) -> HashMap<String, Vec<Connection>> {
    let mut connections_by_pid = HashMap::new();

    for connection in connections {
        let pid = connection.pid.to_string();
        let connections = connections_by_pid.entry(pid).or_insert(Vec::new());
        connections.push(connection);
    }

    connections_by_pid
}

#[derive(Debug, Clone, Serialize)]
pub struct Process {
    pub image_name: String,
    pub pid: String,
    pub session_name: String,
    pub session_number: String,
}

// FIXME: This is a mess, windows is a mess :(
//        Output from `tasklist` is not consistent, sometimes the image name is
//        cut off, because the process name is too long...
pub fn parse_processes(input: &str) -> Vec<Process> {
    let mut processes = Vec::new();

    let lines = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .skip(1);

    for line in lines {
        let tokens: Vec<&str> = line.split(",").collect();

        if tokens.len() < 5 {
            continue;
        }

        // prettify field values from `\"value\"` to `value`
        let image_name = tokens[0].trim_matches('"').to_string();
        let pid = tokens[1].trim_matches('"').to_string();
        let session_name = tokens[2].trim_matches('"').to_string();
        let session_number = tokens[3].trim_matches('"').to_string();

        processes.push(Process {
            image_name,
            pid,
            session_name,
            session_number,
        });
    }

    processes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_stats() {
        let input = r#"
        Interface Statistics

        Received            Sent

        Bytes                    1279913096        89768387
        Unicast packets              961149          234136
        Non-unicast packets            2660          325731
        Discards                          0               0
        Errors                            0               0
        Unknown protocols                 0

        IPv4 Statistics

        Packets Received                   = 60090845
        Received Header Errors             = 0
        Received Address Errors            = 0
        Datagrams Forwarded                = 0
        Unknown Protocols Received         = 0
        Received Packets Discarded         = 25269
        Received Packets Delivered         = 60259935
        Output Requests                    = 30189260
        Routing Discards                   = 0
        Discarded Output Packets           = 21685
        Output Packet No Route             = 403
        Reassembly Required                = 3
        Reassembly Successful              = 1
        Reassembly Failures                = 0
        Datagrams Successfully Fragmented  = 0
        Datagrams Failing Fragmentation    = 0
        Fragments Created                  = 0

        IPv6 Statistics

        Packets Received                   = 154566
        Received Header Errors             = 0
        Received Address Errors            = 1262
        Datagrams Forwarded                = 0
        Unknown Protocols Received         = 0
        Received Packets Discarded         = 5824
        Received Packets Delivered         = 175651
        Output Requests                    = 190416
        Routing Discards                   = 0
        Discarded Output Packets           = 3039
        Output Packet No Route             = 0
        Reassembly Required                = 0
        Reassembly Successful              = 0
        Reassembly Failures                = 0
        Datagrams Successfully Fragmented  = 0
        Datagrams Failing Fragmentation    = 0
        Fragments Created                  = 0

        ICMPv4 Statistics

            Received    Sent
        Messages                  16443       22923
        Errors                    0           0
        Destination Unreachable   16442       22923
        Time Exceeded             1           0
        Parameter Problems        0           0
        Source Quenches           0           0
        Redirects                 0           0
        Echo Replies              0           0
        Echos                     0           0
        Timestamps                0           0
        Timestamp Replies         0           0
        Address Masks             0           0
        Address Mask Replies      0           0
        Router Solicitations      0           0
        Router Advertisements     0           0

        ICMPv6 Statistics

            Received    Sent
        Messages                  19333       36465
        Errors                    0           0
        Destination Unreachable   499         4590
        Packet Too Big            0           0
        Time Exceeded             0           0
        Parameter Problems        0           0
        Echos                     0           0
        Echo Replies              0           0
        MLD Queries               0           0
        MLD Reports               0           0
        MLD Dones                 0           0
        Router Solicitations      0           212
        Router Advertisements     1500        0
        Neighbor Solicitations    8344        23234
        Neighbor Advertisements   8990        8429
        Redirects                 0           0
        Router Renumberings       0           0

        TCP Statistics for IPv4

        Active Opens                        = 64644
        Passive Opens                       = 7674
        Failed Connection Attempts          = 5800
        Reset Connections                   = 12164
        Current Connections                 = 24
        Segments Received                   = 19099006
        Segments Sent                       = 18364045
        Segments Retransmitted              = 24199

        TCP Statistics for IPv6

        Active Opens                        = 2103
        Passive Opens                       = 603
        Failed Connection Attempts          = 5927
        Reset Connections                   = 519
        Current Connections                 = 0
        Segments Received                   = 104587
        Segments Sent                       = 100502
        Segments Retransmitted              = 4152

        UDP Statistics for IPv4

        Datagrams Received    = 53860170
        No Ports              = 25351
        Receive Errors        = 26
        Datagrams Sent        = 24325877

        UDP Statistics for IPv6

        Datagrams Received    = 234072
        No Ports              = 5641
        Receive Errors        = 9
        Datagrams Sent        = 147179
        "#;

        let stats = parse_stats(input);

        assert_eq!(stats.interface.bytes.received, 1279913096);
        assert_eq!(stats.interface.bytes.sent, 89768387);
        assert_eq!(stats.interface.unicast_packets.received, 961149);
        assert_eq!(stats.interface.unicast_packets.sent, 234136);
        assert_eq!(stats.interface.non_unicast_packets.received, 2660);
        assert_eq!(stats.interface.non_unicast_packets.sent, 325731);
        assert_eq!(stats.interface.discards.received, 0);
        assert_eq!(stats.interface.discards.sent, 0);
        assert_eq!(stats.interface.errors.received, 0);
        assert_eq!(stats.interface.errors.sent, 0);
        assert_eq!(stats.interface.unknown_protocols, 0);

        assert_eq!(stats.ipv4.packets_received, 60090845);
        assert_eq!(stats.ipv4.received_header_errors, 0);
        assert_eq!(stats.ipv4.received_address_errors, 0);
        assert_eq!(stats.ipv4.datagrams_forwarded, 0);
        assert_eq!(stats.ipv4.unknown_protocols_received, 0);
        assert_eq!(stats.ipv4.received_packets_discarded, 25269);
        assert_eq!(stats.ipv4.received_packets_delivered, 60259935);
        assert_eq!(stats.ipv4.output_requests, 30189260);
        assert_eq!(stats.ipv4.routing_discards, 0);
        assert_eq!(stats.ipv4.discarded_output_packets, 21685);
        assert_eq!(stats.ipv4.output_packet_no_route, 403);
        assert_eq!(stats.ipv4.reassembly_required, 3);
        assert_eq!(stats.ipv4.reassembly_successful, 1);
        assert_eq!(stats.ipv4.reassembly_failures, 0);
        assert_eq!(stats.ipv4.datagrams_successfully_fragmented, 0);
        assert_eq!(stats.ipv4.datagrams_failing_fragmentation, 0);
        assert_eq!(stats.ipv4.fragments_created, 0);

        assert_eq!(stats.ipv6.packets_received, 154566);
        assert_eq!(stats.ipv6.received_header_errors, 0);
        assert_eq!(stats.ipv6.received_address_errors, 1262);
        assert_eq!(stats.ipv6.datagrams_forwarded, 0);
        assert_eq!(stats.ipv6.unknown_protocols_received, 0);
        assert_eq!(stats.ipv6.received_packets_discarded, 5824);
        assert_eq!(stats.ipv6.received_packets_delivered, 175651);
        assert_eq!(stats.ipv6.output_requests, 190416);
        assert_eq!(stats.ipv6.routing_discards, 0);
        assert_eq!(stats.ipv6.discarded_output_packets, 3039);
        assert_eq!(stats.ipv6.output_packet_no_route, 0);
        assert_eq!(stats.ipv6.reassembly_required, 0);
        assert_eq!(stats.ipv6.reassembly_successful, 0);
        assert_eq!(stats.ipv6.reassembly_failures, 0);
        assert_eq!(stats.ipv6.datagrams_successfully_fragmented, 0);
        assert_eq!(stats.ipv6.datagrams_failing_fragmentation, 0);
        assert_eq!(stats.ipv6.fragments_created, 0);

        assert_eq!(stats.icmpv4.messages.received, 16443);
        assert_eq!(stats.icmpv4.messages.sent, 22923);
        assert_eq!(stats.icmpv4.errors.received, 0);
        assert_eq!(stats.icmpv4.errors.sent, 0);
        assert_eq!(stats.icmpv4.destination_unreachable.received, 16442);
        assert_eq!(stats.icmpv4.destination_unreachable.sent, 22923);
        assert_eq!(stats.icmpv4.time_exceeded.received, 1);
        assert_eq!(stats.icmpv4.time_exceeded.sent, 0);
        assert_eq!(stats.icmpv4.parameter_problems.received, 0);
        assert_eq!(stats.icmpv4.parameter_problems.sent, 0);
        assert_eq!(stats.icmpv4.source_quenches.received, 0);
        assert_eq!(stats.icmpv4.source_quenches.sent, 0);
        assert_eq!(stats.icmpv4.redirects.received, 0);
        assert_eq!(stats.icmpv4.redirects.sent, 0);
        assert_eq!(stats.icmpv4.echo_replies.received, 0);
        assert_eq!(stats.icmpv4.echo_replies.sent, 0);
        assert_eq!(stats.icmpv4.echos.received, 0);
        assert_eq!(stats.icmpv4.echos.sent, 0);
        assert_eq!(stats.icmpv4.timestamps.received, 0);
        assert_eq!(stats.icmpv4.timestamps.sent, 0);
        assert_eq!(stats.icmpv4.address_masks.received, 0);
        assert_eq!(stats.icmpv4.address_masks.sent, 0);
        assert_eq!(stats.icmpv4.address_mask_replies.received, 0);
        assert_eq!(stats.icmpv4.address_mask_replies.sent, 0);
        assert_eq!(stats.icmpv4.router_solicitations.received, 0);
        assert_eq!(stats.icmpv4.router_solicitations.sent, 0);
        assert_eq!(stats.icmpv4.router_advertisements.received, 0);
        assert_eq!(stats.icmpv4.router_advertisements.sent, 0);

        assert_eq!(stats.icmpv6.messages.received, 19333);
        assert_eq!(stats.icmpv6.messages.sent, 36465);
        assert_eq!(stats.icmpv6.errors.received, 0);
        assert_eq!(stats.icmpv6.errors.sent, 0);
        assert_eq!(stats.icmpv6.destination_unreachable.received, 499);
        assert_eq!(stats.icmpv6.destination_unreachable.sent, 4590);
        assert_eq!(stats.icmpv6.time_exceeded.received, 0);
        assert_eq!(stats.icmpv6.time_exceeded.sent, 0);
        assert_eq!(stats.icmpv6.parameter_problems.received, 0);
        assert_eq!(stats.icmpv6.parameter_problems.sent, 0);
        assert_eq!(stats.icmpv6.redirects.received, 0);
        assert_eq!(stats.icmpv6.redirects.sent, 0);
        assert_eq!(stats.icmpv6.echos.received, 0);
        assert_eq!(stats.icmpv6.echos.sent, 0);
        assert_eq!(stats.icmpv6.echo_replies.received, 0);
        assert_eq!(stats.icmpv6.echo_replies.sent, 0);
        assert_eq!(stats.icmpv6.mld_queries.received, 0);
        assert_eq!(stats.icmpv6.mld_queries.sent, 0);
        assert_eq!(stats.icmpv6.mld_reports.received, 0);
        assert_eq!(stats.icmpv6.mld_reports.sent, 0);
        assert_eq!(stats.icmpv6.mld_dones.received, 0);
        assert_eq!(stats.icmpv6.mld_dones.sent, 0);
        assert_eq!(stats.icmpv6.router_solicitations.received, 0);
        assert_eq!(stats.icmpv6.router_solicitations.sent, 212);
        assert_eq!(stats.icmpv6.router_advertisements.received, 1500);
        assert_eq!(stats.icmpv6.router_advertisements.sent, 0);
        assert_eq!(stats.icmpv6.neighbor_solicitations.received, 8344);
        assert_eq!(stats.icmpv6.neighbor_solicitations.sent, 23234);
        assert_eq!(stats.icmpv6.neighbor_advertisements.received, 8990);
        assert_eq!(stats.icmpv6.neighbor_advertisements.sent, 8429);
        assert_eq!(stats.icmpv6.router_renumberings.received, 0);
        assert_eq!(stats.icmpv6.router_renumberings.sent, 0);

        assert_eq!(stats.tcp_ipv4.active_opens, 64644);
        assert_eq!(stats.tcp_ipv4.passive_opens, 7674);
        assert_eq!(stats.tcp_ipv4.failed_connection_attempts, 5800);
        assert_eq!(stats.tcp_ipv4.reset_connections, 12164);
        assert_eq!(stats.tcp_ipv4.current_connections, 24);
        assert_eq!(stats.tcp_ipv4.segments_received, 19099006);
        assert_eq!(stats.tcp_ipv4.segments_sent, 18364045);
        assert_eq!(stats.tcp_ipv4.segments_retransmitted, 24199);

        assert_eq!(stats.tcp_ipv6.active_opens, 2103);
        assert_eq!(stats.tcp_ipv6.passive_opens, 603);
        assert_eq!(stats.tcp_ipv6.failed_connection_attempts, 5927);
        assert_eq!(stats.tcp_ipv6.reset_connections, 519);
        assert_eq!(stats.tcp_ipv6.current_connections, 0);
        assert_eq!(stats.tcp_ipv6.segments_received, 104587);
        assert_eq!(stats.tcp_ipv6.segments_sent, 100502);
        assert_eq!(stats.tcp_ipv6.segments_retransmitted, 4152);

        assert_eq!(stats.udp_ipv4.datagrams_received, 53860170);
        assert_eq!(stats.udp_ipv4.no_ports, 25351);
        assert_eq!(stats.udp_ipv4.receive_errors, 26);
        assert_eq!(stats.udp_ipv4.datagrams_sent, 24325877);

        assert_eq!(stats.udp_ipv6.datagrams_received, 234072);
        assert_eq!(stats.udp_ipv6.no_ports, 5641);
        assert_eq!(stats.udp_ipv6.receive_errors, 9);
        assert_eq!(stats.udp_ipv6.datagrams_sent, 147179);
    }

    #[test]
    fn test_parse_connections() {
        let input = r#"
        Active Connections

        Proto  Local Address          Foreign Address        State           PID
        TCP    0.0.0.0:135            0.0.0.0:0              LISTENING       1192
        TCP    0.0.0.0:445            0.0.0.0:0              LISTENING       4
        TCP    0.0.0.0:5040           0.0.0.0:0              LISTENING       8812
        TCP    0.0.0.0:5357           0.0.0.0:0              LISTENING       4
        TCP    0.0.0.0:5432           0.0.0.0:0              LISTENING       7292
        TCP    0.0.0.0:8828           0.0.0.0:0              LISTENING       9564
        TCP    0.0.0.0:49664          0.0.0.0:0              LISTENING       660
        TCP    0.0.0.0:49665          0.0.0.0:0              LISTENING       972
        TCP    0.0.0.0:49666          0.0.0.0:0              LISTENING       844
        TCP    0.0.0.0:49667          0.0.0.0:0              LISTENING       1752
        TCP    0.0.0.0:49668          0.0.0.0:0              LISTENING       3720
        TCP    0.0.0.0:49669          0.0.0.0:0              LISTENING       4888
        TCP    0.0.0.0:49670          0.0.0.0:0              LISTENING       672
        TCP    0.0.0.0:57621          0.0.0.0:0              LISTENING       3604
        TCP    0.0.0.0:58002          0.0.0.0:0              LISTENING       3604
        TCP    127.0.0.1:6463         0.0.0.0:0              LISTENING       14488
        TCP    127.0.0.1:57784        127.0.0.1:65001        ESTABLISHED     5124
        TCP    127.0.0.1:57808        0.0.0.0:0              LISTENING       4136
        TCP    127.0.0.1:57808        127.0.0.1:57829        ESTABLISHED     4136
        TCP    127.0.0.1:57829        127.0.0.1:57808        ESTABLISHED     12884
        TCP    127.0.0.1:58024        127.0.0.1:58025        ESTABLISHED     9444
        TCP    127.0.0.1:58025        127.0.0.1:58024        ESTABLISHED     9444
        TCP    127.0.0.1:58026        127.0.0.1:58027        ESTABLISHED     14452
        TCP    127.0.0.1:58027        127.0.0.1:58026        ESTABLISHED     14452
        TCP    127.0.0.1:65001        0.0.0.0:0              LISTENING       5124
        TCP    127.0.0.1:65001        127.0.0.1:57784        ESTABLISHED     5124
        TCP    172.29.112.1:139       0.0.0.0:0              LISTENING       4
        TCP    192.168.56.1:139       0.0.0.0:0              LISTENING       4
        TCP    192.168.100.14:139     0.0.0.0:0              LISTENING       4
        TCP    192.168.100.14:57793   40.115.3.253:443       ESTABLISHED     5236
        TCP    192.168.100.14:58006   34.158.0.131:4070      ESTABLISHED     3604
        TCP    192.168.100.14:58803   34.36.232.77:443       ESTABLISHED     11884
        TCP    192.168.100.14:58805   34.36.232.77:443       ESTABLISHED     3604
        TCP    192.168.100.14:59060   192.99.44.195:443      ESTABLISHED     9444
        TCP    192.168.100.14:59062   34.111.115.192:443     ESTABLISHED     14584
        TCP    192.168.100.14:59075   162.159.134.234:443    ESTABLISHED     14584
        TCP    192.168.100.14:59132   34.107.243.93:443      ESTABLISHED     9444
        TCP    192.168.100.14:59212   216.58.215.110:443     ESTABLISHED     9444
        TCP    192.168.100.14:59220   185.225.248.13:443     ESTABLISHED     9444
        TCP    192.168.100.14:59239   35.186.227.140:443     TIME_WAIT       0
        TCP    192.168.100.14:59269   152.199.19.160:443     ESTABLISHED     15988
        TCP    192.168.100.14:59270   35.186.227.140:443     ESTABLISHED     9444
        TCP    192.168.100.14:59271   13.69.68.64:443        ESTABLISHED     15988
        TCP    192.168.100.14:59274   20.44.229.112:443      ESTABLISHED     11332
        TCP    192.168.100.14:59275   204.79.197.239:443     ESTABLISHED     13276
        TCP    [::]:135               [::]:0                 LISTENING       1192
        TCP    [::]:445               [::]:0                 LISTENING       4
        TCP    [::]:5357              [::]:0                 LISTENING       4
        TCP    [::]:5432              [::]:0                 LISTENING       7292
        TCP    [::]:8828              [::]:0                 LISTENING       9564
        TCP    [::]:49664             [::]:0                 LISTENING       660
        TCP    [::]:49665             [::]:0                 LISTENING       972
        TCP    [::]:49666             [::]:0                 LISTENING       844
        TCP    [::]:49667             [::]:0                 LISTENING       1752
        TCP    [::]:49668             [::]:0                 LISTENING       3720
        TCP    [::]:49669             [::]:0                 LISTENING       4888
        TCP    [::]:49670             [::]:0                 LISTENING       672
        TCP    [::1]:1420             [::]:0                 LISTENING       11524
        TCP    [::1]:1420             [::1]:58750            ESTABLISHED     11524
        TCP    [::1]:58750            [::1]:1420             ESTABLISHED     15716
        UDP    0.0.0.0:1900           *:*                                    3604
        UDP    0.0.0.0:1900           *:*                                    3604
        UDP    0.0.0.0:1900           *:*                                    3604
        UDP    0.0.0.0:3702           *:*                                    10680
        UDP    0.0.0.0:3702           *:*                                    10680
        UDP    0.0.0.0:5050           *:*                                    8812
        UDP    0.0.0.0:5353           *:*                                    3604
        UDP    0.0.0.0:5353           *:*                                    3604
        UDP    0.0.0.0:5353           *:*                                    3604
        UDP    0.0.0.0:5353           *:*                                    3604
        UDP    0.0.0.0:5353           *:*                                    3604
        UDP    0.0.0.0:5353           *:*                                    3604
        UDP    0.0.0.0:5353           *:*                                    2896
        UDP    0.0.0.0:5353           *:*                                    3604
        UDP    0.0.0.0:5353           *:*                                    3604
        UDP    0.0.0.0:5353           *:*                                    3604
        UDP    0.0.0.0:5355           *:*                                    2896
        UDP    0.0.0.0:50250          *:*                                    11884
        UDP    0.0.0.0:52296          *:*                                    11884
        UDP    0.0.0.0:54615          *:*                                    13276
        UDP    0.0.0.0:54868          *:*                                    5124
        UDP    0.0.0.0:57621          *:*                                    3604
        UDP    0.0.0.0:59269          *:*                                    11884
        UDP    0.0.0.0:61972          *:*                                    3604
        UDP    0.0.0.0:61973          *:*                                    3604
        UDP    0.0.0.0:61974          *:*                                    3604
        UDP    0.0.0.0:62015          *:*                                    11884
        UDP    0.0.0.0:63433          *:*                                    15988
        UDP    0.0.0.0:65004          *:*                                    10680
        UDP    127.0.0.1:1900         *:*                                    3920
        UDP    127.0.0.1:10020        *:*                                    4136
        UDP    127.0.0.1:57841        *:*                                    5008
        UDP    127.0.0.1:58331        *:*                                    3920
        UDP    127.0.0.1:64719        *:*                                    11480
        UDP    172.29.112.1:137       *:*                                    4
        UDP    172.29.112.1:138       *:*                                    4
        UDP    172.29.112.1:1900      *:*                                    3920
        UDP    172.29.112.1:2177      *:*                                    12060
        UDP    172.29.112.1:5353      *:*                                    5124
        UDP    172.29.112.1:54616     *:*                                    15768
        UDP    172.29.112.1:64724     *:*                                    3920
        UDP    192.168.56.1:137       *:*                                    4
        UDP    192.168.56.1:138       *:*                                    4
        UDP    192.168.56.1:1900      *:*                                    3920
        UDP    192.168.56.1:2177      *:*                                    12060
        UDP    192.168.56.1:5353      *:*                                    5124
        UDP    192.168.56.1:54617     *:*                                    15768
        UDP    192.168.56.1:64725     *:*                                    3920
        UDP    192.168.100.14:137     *:*                                    4
        UDP    192.168.100.14:138     *:*                                    4
        UDP    192.168.100.14:1900    *:*                                    3920
        UDP    192.168.100.14:2177    *:*                                    12060
        UDP    192.168.100.14:5353    *:*                                    5124
        UDP    192.168.100.14:54618   *:*                                    15768
        UDP    192.168.100.14:58330   *:*                                    3920
        UDP    [::]:3702              *:*                                    10680
        UDP    [::]:3702              *:*                                    10680
        UDP    [::]:5353              *:*                                    3604
        UDP    [::]:5353              *:*                                    3604
        UDP    [::]:5353              *:*                                    3604
        UDP    [::]:5353              *:*                                    3604
        UDP    [::]:5353              *:*                                    3604
        UDP    [::]:5353              *:*                                    2896
        UDP    [::]:5353              *:*                                    3604
        UDP    [::]:5355              *:*                                    2896
        UDP    [::]:54869             *:*                                    5124
        UDP    [::]:65005             *:*                                    10680
        UDP    [::1]:1900             *:*                                    3920
        UDP    [::1]:5353             *:*                                    5124
        UDP    [::1]:57848            *:*                                    7292
        UDP    [::1]:64723            *:*                                    3920
        UDP    [fe80::2eab:cf7c:88:b019%17]:1900  *:*                                    3920
        UDP    [fe80::2eab:cf7c:88:b019%17]:2177  *:*                                    12060
        UDP    [fe80::2eab:cf7c:88:b019%17]:64721  *:*                                    3920
        UDP    [fe80::d780:dafb:bd15:aba8%3]:1900  *:*                                    3920
        UDP    [fe80::d780:dafb:bd15:aba8%3]:2177  *:*                                    12060
        UDP    [fe80::d780:dafb:bd15:aba8%3]:64720  *:*                                    3920
        UDP    [fe80::f28d:4249:ac12:d039%16]:1900  *:*                                    3920
        UDP    [fe80::f28d:4249:ac12:d039%16]:2177  *:*                                    12060
        UDP    [fe80::f28d:4249:ac12:d039%16]:64722  *:*                                    3920"#;

        let connections = parse_connections(input);

        println!("Connections: {}", connections.len());

        for connection in connections {
            println!("{:?}", connection);
        }
    }

    #[test]
    fn test_parse_processes() {
        let input = r#"
        "Image Name","PID","Session Name","Session#","Mem Usage"
        "System Idle Process","0","Services","0","8 K"
        "System","4","Services","0","144 K"
        "Registry","172","Services","0","84 496 K"
        "smss.exe","544","Services","0","1 052 K"
        "csrss.exe","880","Services","0","5 632 K"
        "wininit.exe","964","Services","0","6 976 K"
        "csrss.exe","976","Console","1","6 236 K"
        "services.exe","632","Services","0","13 208 K"
        "lsass.exe","668","Services","0","22 568 K"
        "svchost.exe","1056","Services","0","28 256 K"
        "fontdrvhost.exe","1084","Services","0","2 692 K"
        "svchost.exe","1144","Services","0","15 812 K"
        "svchost.exe","1200","Services","0","10 816 K"
        "winlogon.exe","1252","Console","1","12 312 K"
        "fontdrvhost.exe","1320","Console","1","5 628 K"
        "dwm.exe","1396","Console","1","52 620 K"
        "svchost.exe","1472","Services","0","11 644 K"
        "svchost.exe","1552","Services","0","5 176 K"
        "svchost.exe","1624","Services","0","17 536 K"
        "svchost.exe","1644","Services","0","15 604 K"
        "svchost.exe","1688","Services","0","10 524 K"
        "svchost.exe","1696","Services","0","13 828 K"
        "svchost.exe","1704","Services","0","11 996 K"
        "svchost.exe","1836","Services","0","7 396 K"
        "svchost.exe","1864","Services","0","11 516 K"
        "svchost.exe","1896","Services","0","6 500 K"
        "svchost.exe","1936","Services","0","7 652 K"
        "svchost.exe","2000","Services","0","6 028 K"
        "svchost.exe","2072","Services","0","11 096 K"
        "svchost.exe","2108","Services","0","11 432 K"
        "svchost.exe","2200","Services","0","10 120 K"
        "svchost.exe","2312","Services","0","7 104 K"
        "svchost.exe","2372","Services","0","7 440 K"
        "NVDisplay.Container.exe","2484","Services","0","19 192 K"
        "svchost.exe","2492","Services","0","8 224 K"
        "svchost.exe","2704","Services","0","5 944 K"
        "svchost.exe","2712","Services","0","14 340 K"
        "svchost.exe","2720","Services","0","7 848 K"
        "NVDisplay.Container.exe","2796","Console","1","51 400 K"
        "Memory Compression","2836","Services","0","554 360 K"
        "svchost.exe","2872","Services","0","8 104 K"
        "svchost.exe","2904","Services","0","18 180 K"
        "svchost.exe","2928","Services","0","8 084 K"
        "svchost.exe","2936","Services","0","9 348 K"
        "svchost.exe","2156","Services","0","7 676 K"
        "svchost.exe","3092","Services","0","6 812 K"
        "svchost.exe","3264","Services","0","8 196 K"
        "svchost.exe","3320","Services","0","8 716 K"
        "svchost.exe","3600","Services","0","8 620 K"
        "svchost.exe","3956","Services","0","14 436 K"
        "svchost.exe","4064","Services","0","17 924 K"
        "svchost.exe","3224","Services","0","10 092 K"
        "svchost.exe","3216","Services","0","6 852 K"
        "svchost.exe","3876","Services","0","58 240 K"
        "svchost.exe","3892","Services","0","27 360 K"
        "svchost.exe","2996","Services","0","18 040 K"
        "svchost.exe","4240","Services","0","16 932 K"
        "svchost.exe","4296","Services","0","10 540 K"
        "svchost.exe","4304","Services","0","14 392 K"
        "svchost.exe","4316","Services","0","13 184 K"
        "spoolsv.exe","4584","Services","0","14 148 K"
        "svchost.exe","4812","Services","0","6 368 K"
        "svchost.exe","4820","Services","0","13 636 K"
        "svchost.exe","4828","Services","0","33 888 K"
        "svchost.exe","4836","Services","0","6 420 K"
        "RtkAudUService64.exe","4844","Services","0","10 144 K"
        "svchost.exe","4852","Services","0","35 404 K"
        "RtkBtManServ.exe","4860","Services","0","6 904 K"
        "svchost.exe","4868","Services","0","8 372 K"
        "svchost.exe","4876","Services","0","5 728 K"
        "svchost.exe","4884","Services","0","20 920 K"
        "pg_ctl.exe","4892","Services","0","6 004 K"
        "sqlwriter.exe","4900","Services","0","8 072 K"
        "nvcontainer.exe","4908","Services","0","38 580 K"
        "MsMpEng.exe","4940","Services","0","308 528 K"
        "wslservice.exe","4956","Services","0","19 124 K"
        "MBAMService.exe","4968","Services","0","59 264 K"
        "svchost.exe","5064","Services","0","7 248 K"
        "svchost.exe","5040","Services","0","5 428 K"
        "dasHost.exe","5176","Services","0","12 724 K"
        "svchost.exe","5568","Services","0","9 356 K"
        "com.docker.service","5760","Services","0","34 120 K"
        "svchost.exe","5768","Services","0","13 384 K"
        "wallpaperservice32_c.exe","5892","Services","0","5 368 K"
        "rundll32.exe","6208","Console","1","6 596 K"
        "svchost.exe","6268","Services","0","7 624 K"
        "SearchIndexer.exe","6884","Services","0","46 984 K"
        "svchost.exe","7120","Services","0","22 120 K"
        "postgres.exe","6416","Services","0","19 460 K"
        "conhost.exe","3112","Services","0","7 116 K"
        "postgres.exe","6848","Services","0","7 156 K"
        "postgres.exe","1048","Services","0","9 056 K"
        "postgres.exe","7284","Services","0","8 680 K"
        "postgres.exe","7304","Services","0","11 724 K"
        "postgres.exe","7324","Services","0","8 172 K"
        "postgres.exe","7344","Services","0","7 540 K"
        "postgres.exe","7364","Services","0","7 880 K"
        "svchost.exe","7724","Services","0","7 172 K"
        "svchost.exe","7800","Services","0","8 980 K"
        "NisSrv.exe","8128","Services","0","11 308 K"
        "mbamtray.exe","2640","Console","1","40 260 K"
        "nvcontainer.exe","8220","Console","1","27 320 K"
        "nvcontainer.exe","8256","Console","1","45 688 K"
        "sihost.exe","8268","Console","1","27 068 K"
        "svchost.exe","8320","Console","1","33 248 K"
        "svchost.exe","8364","Console","1","36 128 K"
        "taskhostw.exe","8496","Console","1","17 980 K"
        "svchost.exe","8556","Services","0","22 184 K"
        "wallpaper64.exe","8608","Console","1","14 488 K"
        "svchost.exe","8880","Services","0","8 080 K"
        "explorer.exe","8888","Console","1","179 952 K"
        "ctfmon.exe","8952","Console","1","15 996 K"
        "svchost.exe","9064","Services","0","18 004 K"
        "rundll32.exe","9476","Console","1","8 720 K"
        "svchost.exe","9520","Console","1","20 560 K"
        "NVIDIA Web Helper.exe","9684","Console","1","17 168 K"
        "StartMenuExperienceHost.exe","9848","Console","1","68 628 K"
        "taskhostw.exe","9908","Console","1","17 336 K"
        "conhost.exe","9976","Console","1","816 K"
        "RuntimeBroker.exe","10080","Console","1","26 336 K"
        "SearchApp.exe","8568","Console","1","94 680 K"
        "RuntimeBroker.exe","10616","Console","1","28 344 K"
        "RuntimeBroker.exe","10432","Console","1","21 392 K"
        "svchost.exe","11468","Services","0","12 140 K"
        "dllhost.exe","10484","Console","1","13 120 K"
        "nvsphelper64.exe","12796","Console","1","13 036 K"
        "NVIDIA Share.exe","12816","Console","1","54 984 K"
        "NVIDIA Share.exe","13016","Console","1","33 096 K"
        "NVIDIA Share.exe","13088","Console","1","54 648 K"
        "TextInputHost.exe","12444","Console","1","45 796 K"
        "SecurityHealthSystray.exe","12480","Console","1","9 392 K"
        "SecurityHealthService.exe","12232","Services","0","16 160 K"
        "RtkAudUService64.exe","12276","Console","1","8 212 K"
        "Discord.exe","12776","Console","1","77 416 K"
        "Discord.exe","13392","Console","1","26 556 K"
        "Discord.exe","13756","Console","1","59 648 K"
        "Discord.exe","13820","Console","1","43 728 K"
        "svchost.exe","14164","Services","0","12 504 K"
        "Discord.exe","14572","Console","1","202 380 K"
        "Discord.exe","14996","Console","1","63 300 K"
        "svchost.exe","900","Services","0","9 788 K"
        "Code.exe","14820","Console","1","98 772 K"
        "Code.exe","15212","Console","1","111 480 K"
        "Code.exe","15336","Console","1","40 732 K"
        "Code.exe","1912","Console","1","228 856 K"
        "Code.exe","14364","Console","1","180 076 K"
        "Code.exe","6300","Console","1","98 028 K"
        "Code.exe","6876","Console","1","74 256 K"
        "Code.exe","14416","Console","1","78 040 K"
        "conhost.exe","11040","Console","1","6 476 K"
        "cmd.exe","1412","Console","1","5 096 K"
        "rust-analyzer.exe","11124","Console","1","904 396 K"
        "conhost.exe","9960","Console","1","5 864 K"
        "powershell.exe","9504","Console","1","53 824 K"
        "conhost.exe","10884","Console","1","5 880 K"
        "Code.exe","15480","Console","1","74 656 K"
        "Code.exe","16372","Console","1","140 288 K"
        "Code.exe","13100","Console","1","79 400 K"
        "rust-analyzer-proc-macro-srv.exe","13032","Console","1","23 004 K"
        "dllhost.exe","16028","Services","0","11 512 K"
        "ApplicationFrameHost.exe","14684","Console","1","29 276 K"
        "CalculatorApp.exe","13676","Console","1","1 996 K"
        "RuntimeBroker.exe","3984","Console","1","7 296 K"
        "SystemSettings.exe","4672","Console","1","2 544 K"
        "UserOOBEBroker.exe","16152","Console","1","10 204 K"
        "SgrmBroker.exe","2644","Services","0","8 300 K"
        "svchost.exe","4424","Services","0","10 728 K"
        "svchost.exe","14176","Console","1","12 720 K"
        "svchost.exe","9368","Services","0","19 708 K"
        "ShellExperienceHost.exe","4336","Console","1","55 800 K"
        "RuntimeBroker.exe","15372","Console","1","19 468 K"
        "firefox.exe","3740","Console","1","547 792 K"
        "firefox.exe","11224","Console","1","120 548 K"
        "firefox.exe","6572","Console","1","16 800 K"
        "firefox.exe","4404","Console","1","91 072 K"
        "firefox.exe","6100","Console","1","289 864 K"
        "firefox.exe","11600","Console","1","15 816 K"
        "firefox.exe","13948","Console","1","174 772 K"
        "firefox.exe","11812","Console","1","183 952 K"
        "firefox.exe","272","Console","1","105 820 K"
        "firefox.exe","2060","Console","1","142 784 K"
        "firefox.exe","9600","Console","1","220 124 K"
        "svchost.exe","13628","Services","0","13 160 K"
        "svchost.exe","1572","Services","0","7 588 K"
        "svchost.exe","6800","Services","0","10 708 K"
        "Video.UI.exe","9372","Console","1","9 008 K"
        "RuntimeBroker.exe","3308","Console","1","8 280 K"
        "firefox.exe","10000","Console","1","248 000 K"
        "firefox.exe","6708","Console","1","163 936 K"
        "firefox.exe","10608","Console","1","124 108 K"
        "firefox.exe","14900","Console","1","175 224 K"
        "Code.exe","2468","Console","1","153 552 K"
        "firefox.exe","8236","Console","1","204 088 K"
        "firefox.exe","11132","Console","1","157 804 K"
        "firefox.exe","13672","Console","1","193 172 K"
        "firefox.exe","828","Console","1","36 284 K"
        "svchost.exe","13484","Services","0","8 052 K"
        "msedge.exe","7008","Console","1","141 624 K"
        "msedge.exe","10796","Console","1","8 068 K"
        "msedge.exe","1816","Console","1","36 192 K"
        "msedge.exe","11844","Console","1","31 516 K"
        "msedge.exe","10372","Console","1","18 236 K"
        "svchost.exe","15592","Services","0","7 612 K"
        "node.exe","10824","Console","1","70 080 K"
        "cmd.exe","5000","Console","1","4 968 K"
        "node.exe","4260","Console","1","44 808 K"
        "cmd.exe","8480","Console","1","5 000 K"
        "node.exe","6792","Console","1","70 100 K"
        "cmd.exe","6732","Console","1","4 976 K"
        "node.exe","15744","Console","1","186 200 K"
        "esbuild.exe","5520","Console","1","14 756 K"
        "firefox.exe","8432","Console","1","36 100 K"
        "firefox.exe","15584","Console","1","36 156 K"
        "SearchProtocolHost.exe","14000","Services","0","13 808 K"
        "SearchFilterHost.exe","6604","Services","0","7 744 K"
        "RuntimeBroker.exe","1044","Console","1","22 152 K"
        "net-process.exe","7336","Console","1","25 180 K"
        "msedgewebview2.exe","16172","Console","1","100 988 K"
        "msedgewebview2.exe","6172","Console","1","7 588 K"
        "msedgewebview2.exe","10592","Console","1","55 420 K"
        "msedgewebview2.exe","680","Console","1","29 020 K"
        "msedgewebview2.exe","32","Console","1","17 580 K"
        "msedgewebview2.exe","5700","Console","1","74 888 K"
        "WmiPrvSE.exe","10480","Services","0","11 540 K"
        "WindowsTerminal.exe","5052","Console","1","80 980 K"
        "dllhost.exe","10292","Console","1","7 684 K"
        "OpenConsole.exe","7028","Console","1","8 968 K"
        "pwsh.exe","10132","Console","1","102 692 K"
        "RuntimeBroker.exe","12044","Console","1","13 064 K"
        "tasklist.exe","1468","Console","1","9 440 K"
        "#;

        let processes = parse_processes(input);

        assert_eq!(processes.len(), 200);
    }
}
