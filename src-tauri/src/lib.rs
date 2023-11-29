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
        .skip(5);

    for line in lines {
        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.len() < 3 {
            continue;
        }

        if tokens[0].contains(".") {
            let image_name = tokens[0].to_string();
            let pid = tokens[1].to_string();
            let session_name = tokens[2].to_string();
            processes.push(Process {
                image_name,
                pid,
                session_name,
            });
            continue;
        }

        let mut image_name_pieces = Vec::new();
        let mut image_name_end = 0;
        for &token in tokens.iter().take_while(|token| !token.contains(".")) {
            image_name_pieces.push(token);
            image_name_end += 1;
        }

        let pid = tokens[image_name_end - 1].to_string();
        let session_name = tokens[image_name_end - 1].to_string();

        let mut image_name = image_name_pieces.join(" ");
        image_name.push_str(".exe");

        // If the pid is not a number, skip this process
        // Why we need to do this? Because windows is a mess...
        if u128::from_str_radix(&pid, 10).is_err() {
            continue;
        }

        processes.push(Process {
            image_name,
            pid,
            session_name,
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
        Image Name                     PID Session Name        Session#    Mem Usage
        ========================= ======== ================ =========== ============
        System Idle Process              0 Services                   0          8 K
        System                           4 Services                   0        144 K
        Registry                       172 Services                   0    106 440 K
        smss.exe                       548 Services                   0      1 112 K
        csrss.exe                      884 Services                   0      5 836 K
        wininit.exe                    972 Services                   0      6 740 K
        services.exe                   672 Services                   0     12 176 K
        lsass.exe                      660 Services                   0     20 660 K
        svchost.exe                   1104 Services                   0     27 792 K
        fontdrvhost.exe               1132 Services                   0      3 056 K
        svchost.exe                   1192 Services                   0     15 792 K
        svchost.exe                   1248 Services                   0     10 516 K
        svchost.exe                   1520 Services                   0     10 640 K
        svchost.exe                   1600 Services                   0      7 512 K
        svchost.exe                   1608 Services                   0     11 504 K
        svchost.exe                   1624 Services                   0     11 644 K
        svchost.exe                   1752 Services                   0     15 000 K
        svchost.exe                   1776 Services                   0     10 056 K
        svchost.exe                   1784 Services                   0     13 216 K
        svchost.exe                   1796 Services                   0     11 544 K
        svchost.exe                   1984 Services                   0     11 416 K
        svchost.exe                   2004 Services                   0      5 976 K
        svchost.exe                    844 Services                   0     15 664 K
        svchost.exe                   1704 Services                   0      8 124 K
        svchost.exe                   2052 Services                   0     10 132 K
        svchost.exe                   2136 Services                   0      5 712 K
        svchost.exe                   2252 Services                   0      6 984 K
        svchost.exe                   2344 Services                   0      7 212 K
        svchost.exe                   2424 Services                   0     11 384 K
        svchost.exe                   2452 Services                   0      7 844 K
        dasHost.exe                   2556 Services                   0     13 472 K
        svchost.exe                   2576 Services                   0      9 992 K
        svchost.exe                   2616 Services                   0      6 664 K
        svchost.exe                   2648 Services                   0     11 120 K
        svchost.exe                   2780 Services                   0     17 784 K
        NVDisplay.Container.exe       2812 Services                   0     18 920 K
        svchost.exe                   2820 Services                   0      7 308 K
        svchost.exe                   2896 Services                   0      8 624 K
        svchost.exe                   2960 Services                   0      5 516 K
        svchost.exe                   2968 Services                   0     14 092 K
        svchost.exe                   2976 Services                   0      7 172 K
        Memory Compression            2164 Services                   0    287 860 K
        svchost.exe                   2656 Services                   0      7 840 K
        svchost.exe                   2920 Services                   0      7 608 K
        svchost.exe                   3020 Services                   0      9 192 K
        svchost.exe                   3240 Services                   0     16 888 K
        svchost.exe                   3344 Services                   0      7 092 K
        svchost.exe                   3424 Services                   0      6 684 K
        svchost.exe                   3632 Services                   0      7 480 K
        svchost.exe                   3720 Services                   0      7 852 K
        svchost.exe                   3920 Services                   0      6 712 K
        svchost.exe                   3952 Services                   0      8 280 K
        svchost.exe                   4344 Services                   0     15 324 K
        svchost.exe                   4484 Services                   0      9 520 K
        svchost.exe                   4492 Services                   0      6 448 K
        svchost.exe                   4664 Services                   0     16 316 K
        svchost.exe                   4748 Services                   0     16 292 K
        svchost.exe                   4828 Services                   0     13 060 K
        spoolsv.exe                   4888 Services                   0     12 676 K
        svchost.exe                   5008 Services                   0      9 624 K
        svchost.exe                   4644 Services                   0     12 776 K
        RtkAudUService64.exe          4680 Services                   0      9 088 K
        nvcontainer.exe               5124 Services                   0     37 828 K
        svchost.exe                   5140 Services                   0      5 780 K
        svchost.exe                   5148 Services                   0     35 732 K
        MsMpEng.exe                   5156 Services                   0    351 740 K
        RtkBtManServ.exe              5164 Services                   0      6 420 K
        svchost.exe                   5172 Services                   0      7 568 K
        svchost.exe                   5180 Services                   0     27 228 K
        pg_ctl.exe                    5212 Services                   0      5 268 K
        svchost.exe                   5228 Services                   0      5 220 K
        svchost.exe                   5236 Services                   0     20 512 K
        wslservice.exe                5248 Services                   0     16 476 K
        MBAMService.exe               5288 Services                   0     58 212 K
        sqlwriter.exe                 5296 Services                   0      7 496 K
        svchost.exe                   5424 Services                   0      6 548 K
        svchost.exe                   5608 Services                   0      5 032 K
        wallpaperservice32_c.exe      5896 Services                   0      4 956 K
        SearchIndexer.exe             5936 Services                   0     49 932 K
        svchost.exe                   6048 Services                   0      9 792 K
        svchost.exe                   4528 Services                   0      8 736 K
        svchost.exe                   6164 Services                   0     11 964 K
        com.docker.service            6268 Services                   0     31 652 K
        postgres.exe                  7292 Services                   0     18 520 K
        conhost.exe                   7348 Services                   0      6 988 K
        postgres.exe                  7556 Services                   0      6 744 K
        postgres.exe                  7752 Services                   0      8 300 K
        postgres.exe                  7760 Services                   0      8 108 K
        postgres.exe                  7780 Services                   0     11 116 K
        postgres.exe                  7800 Services                   0      7 672 K
        postgres.exe                  7832 Services                   0      7 092 K
        postgres.exe                  7852 Services                   0      7 372 K
        NisSrv.exe                    7980 Services                   0     11 868 K
        svchost.exe                   8424 Services                   0     16 916 K
        svchost.exe                   8432 Services                   0      7 596 K
        svchost.exe                   8812 Services                   0     21 204 K
        svchost.exe                   8724 Services                   0     22 888 K
        svchost.exe                  10536 Services                   0      6 624 K
        svchost.exe                  10680 Services                   0      8 200 K
        dllhost.exe                   9348 Services                   0     10 276 K
        svchost.exe                  14000 Services                   0     16 916 K
        svchost.exe                  10256 Services                   0     12 428 K
        SecurityHealthService.exe    13508 Services                   0     16 720 K
        svchost.exe                  15096 Services                   0     12 348 K
        audiodg.exe                   7572 Services                   0     17 104 K
        svchost.exe                  15652 Services                   0      9 096 K
        SgrmBroker.exe               14324 Services                   0      7 804 K
        svchost.exe                  14316 Services                   0     10 280 K
        svchost.exe                  12060 Services                   0      6 104 K
        svchost.exe                   5208 Services                   0     12 172 K
        svchost.exe                   3556 Services                   0      9 712 K
        csrss.exe                    13196 Console                    3      6 484 K
        winlogon.exe                 10024 Console                    3     10 620 K
        fontdrvhost.exe              17236 Console                    3      5 856 K
        dwm.exe                       8612 Console                    3     54 756 K
        NVDisplay.Container.exe       6608 Console                    3     50 272 K
        svchost.exe                  12296 Services                   0      5 588 K
        mbamtray.exe                 10872 Console                    3     42 756 K
        ctfmon.exe                   16948 Console                    3     20 668 K
        sihost.exe                   17360 Console                    3     26 572 K
        svchost.exe                  16228 Console                    3      8 584 K
        svchost.exe                  16944 Console                    3     28 572 K
        nvcontainer.exe              13128 Console                    3     28 832 K
        wallpaper64.exe              16996 Console                    3     14 604 K
        svchost.exe                   7532 Console                    3     37 540 K
        nvcontainer.exe              17092 Console                    3     44 772 K
        taskhostw.exe                 4476 Console                    3     18 236 K
        explorer.exe                  5468 Console                    3    132 808 K
        svchost.exe                  14304 Console                    3     21 236 K
        NVIDIA Web Helper.exe         3368 Console                    3     25 164 K
        conhost.exe                   9652 Console                    3      1 144 K
        StartMenuExperienceHost.e     2120 Console                    3     68 060 K
        RuntimeBroker.exe             7908 Console                    3     23 036 K
        SearchApp.exe                12704 Console                    3     99 220 K
        RuntimeBroker.exe             9712 Console                    3     33 896 K
        RuntimeBroker.exe            16064 Console                    3     20 984 K
        dllhost.exe                    696 Console                    3     14 572 K
        nvsphelper64.exe             13124 Console                    3     13 252 K
        NVIDIA Share.exe             17404 Console                    3     56 752 K
        NVIDIA Share.exe              3620 Console                    3     32 316 K
        NVIDIA Share.exe              7152 Console                    3     54 432 K
        TextInputHost.exe             8396 Console                    3     46 156 K
        SecurityHealthSystray.exe     9124 Console                    3      9 592 K
        RtkAudUService64.exe         12308 Console                    3      8 796 K
        Discord.exe                  13724 Console                    3     81 808 K
        msedge.exe                   15372 Console                    3     84 204 K
        msedge.exe                   14788 Console                    3      7 476 K
        msedge.exe                   15936 Console                    3     31 692 K
        msedge.exe                    3676 Console                    3     29 696 K
        msedge.exe                    5828 Console                    3     15 840 K
        Discord.exe                   8564 Console                    3     26 760 K
        Discord.exe                  13476 Console                    3    132 252 K
        Discord.exe                   2128 Console                    3     44 596 K
        Discord.exe                  12648 Console                    3    278 504 K
        Discord.exe                   8452 Console                    3     63 320 K
        ApplicationFrameHost.exe      8948 Console                    3     25 692 K
        CalculatorApp.exe             9648 Console                    3      1 956 K
        RuntimeBroker.exe             3044 Console                    3      7 148 K
        svchost.exe                  14260 Console                    3     12 252 K
        firefox.exe                   9984 Console                    3    497 776 K
        firefox.exe                  14384 Console                    3    150 588 K
        firefox.exe                  13848 Console                    3     14 412 K
        firefox.exe                   4568 Console                    3    101 896 K
        firefox.exe                   2768 Console                    3    278 036 K
        firefox.exe                  14776 Console                    3     15 496 K
        firefox.exe                  11784 Console                    3    262 568 K
        firefox.exe                   4292 Console                    3    540 932 K
        firefox.exe                   6940 Console                    3     25 796 K
        firefox.exe                  14664 Console                    3     17 132 K
        firefox.exe                  15120 Console                    3     38 792 K
        ShellExperienceHost.exe       1284 Console                    3     46 756 K
        RuntimeBroker.exe            11856 Console                    3     19 608 K
        UserOOBEBroker.exe            2808 Console                    3     10 080 K
        firefox.exe                  14800 Console                    3    312 856 K
        Spotify.exe                   9964 Console                    3    224 068 K
        Spotify.exe                   8064 Console                    3     17 368 K
        Spotify.exe                  12404 Console                    3    118 300 K
        Spotify.exe                  14764 Console                    3     26 896 K
        Spotify.exe                  10944 Console                    3     42 412 K
        Spotify.exe                  15108 Console                    3    217 716 K
        Code.exe                     13280 Console                    3    101 376 K
        Code.exe                     14984 Console                    3     94 628 K
        Code.exe                     13612 Console                    3     41 500 K
        Code.exe                      8308 Console                    3    191 004 K
        Code.exe                      8832 Console                    3    220 656 K
        Code.exe                      9724 Console                    3     98 676 K
        Code.exe                      7476 Console                    3     75 636 K
        Code.exe                     11076 Console                    3     85 628 K
        conhost.exe                   1452 Console                    3      6 780 K
        cmd.exe                       2872 Console                    3      4 252 K
        rust-analyzer.exe            15764 Console                    3  1 858 952 K
        conhost.exe                  15132 Console                    3     11 464 K
        powershell.exe               14220 Console                    3     78 668 K
        conhost.exe                  14188 Console                    3      8 104 K
        Code.exe                      9132 Console                    3     80 016 K
        Code.exe                      4112 Console                    3     98 888 K
        rust-analyzer-proc-macro-     1552 Console                    3     22 964 K
        firefox.exe                  11208 Console                    3     37 144 K
        svchost.exe                  12144 Services                   0      7 620 K
        wgc.exe                      16340 Console                    3    106 620 K
        WargamingErrorMonitor.exe     2332 Console                    3     12 844 K
        SearchProtocolHost.exe       12564 Services                   0     13 684 K
        SearchFilterHost.exe          2804 Services                   0      7 756 K
        WindowsTerminal.exe           9940 Console                    3     82 708 K
        OpenConsole.exe               1200 Console                    3      9 060 K
        pwsh.exe                     17152 Console                    3    104 796 K
        RuntimeBroker.exe            14924 Console                    3     13 056 K
        wgc_renderer_host.exe         9256 Console                    3     81 440 K
        WmiPrvSE.exe                 16336 Services                   0     14 020 K
        wgc_renderer_host.exe         4164 Console                    3     31 088 K
        svchost.exe                  14600 Services                   0     12 220 K
        wgc_renderer_host.exe        12096 Console                    3     23 236 K
        wgc_renderer_host.exe         2356 Console                    3     30 156 K
        wgc_renderer_host.exe        13752 Console                    3    100 400 K
        WorldOfTanks.exe              7144 Console                    3    513 920 K
        WargamingErrorMonitor.exe    12248 Console                    3     11 460 K
        firefox.exe                   3528 Console                    3     36 144 K
        svchost.exe                   9820 Services                   0     10 068 K
        firefox.exe                  16784 Console                    3     36 136 K
        tasklist.exe                 12128 Console                    3      9 432 K
        "#;

        let processes = parse_processes(input);

        assert_eq!(processes.len(), 200);
    }
}
