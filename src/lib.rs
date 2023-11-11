#[derive(Debug)]
pub struct NetworkData {
    received: u64,
    sent: u64,
}

#[derive(Debug)]
pub struct InterfaceStats {
    bytes: NetworkData,
    unicast_packets: NetworkData,
    non_unicast_packets: NetworkData,
    discards: NetworkData,
    errors: NetworkData,
    unknown_protocols: u64,
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct UdpV4Statistics {
    datagrams_received: u64,
    no_ports: u64,
    receive_errors: u64,
    datagrams_sent: u64,
}

#[derive(Debug)]
pub struct UdpV6Statistics {
    datagrams_received: u64,
    no_ports: u64,
    receive_errors: u64,
    datagrams_sent: u64,
}

#[derive(Debug)]
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
}
