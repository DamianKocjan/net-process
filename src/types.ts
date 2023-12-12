export type NetStatValue = {
  received: number;
  sent: number;
};

export type NetStat = {
  interface: {
    bytes: NetStatValue;
    unicast_packets: NetStatValue;
    non_unicast_packets: NetStatValue;
    discards: NetStatValue;
    errors: NetStatValue;
    unknown_protocols: number;
  };
  ipv4: {
    packets_received: number;
    received_header_errors: number;
    received_address_errors: number;
    datagrams_forwarded: number;
    unknown_protocols_received: number;
    received_packets_discarded: number;
    received_packets_delivered: number;
    output_requests: number;
    routing_discards: number;
    discarded_output_packets: number;
    output_packet_no_route: number;
    reassembly_required: number;
    reassembly_successful: number;
    reassembly_failures: number;
    datagrams_successfully_fragmented: number;
    datagrams_failing_fragmentation: number;
    fragments_created: number;
  };
  ipv6: {
    packets_received: number;
    received_header_errors: number;
    received_address_errors: number;
    datagrams_forwarded: number;
    unknown_protocols_received: number;
    received_packets_discarded: number;
    received_packets_delivered: number;
    output_requests: number;
    routing_discards: number;
    discarded_output_packets: number;
    output_packet_no_route: number;
    reassembly_required: number;
    reassembly_successful: number;
    reassembly_failures: number;
    datagrams_successfully_fragmented: number;
    datagrams_failing_fragmentation: number;
    fragments_created: number;
  };
  icmpv4: {
    messages: NetStatValue;
    errors: NetStatValue;
    destination_unreachable: NetStatValue;
    time_exceeded: NetStatValue;
    parameter_problems: NetStatValue;
    source_quenches: NetStatValue;
    redirects: NetStatValue;
    echo_replies: NetStatValue;
    echos: NetStatValue;
    timestamps: NetStatValue;
    timestamp_replies: NetStatValue;
    address_masks: NetStatValue;
    address_mask_replies: NetStatValue;
    router_solicitations: NetStatValue;
    router_advertisements: NetStatValue;
  };
  icmpv6: {
    messages: NetStatValue;
    errors: NetStatValue;
    destination_unreachable: NetStatValue;
    packet_too_big: NetStatValue;
    time_exceeded: NetStatValue;
    parameter_problems: NetStatValue;
    echos: NetStatValue;
    echo_replies: NetStatValue;
    mld_queries: NetStatValue;
    mld_reports: NetStatValue;
    mld_dones: NetStatValue;
    router_solicitations: NetStatValue;
    router_advertisements: NetStatValue;
    neighbor_solicitations: NetStatValue;
    neighbor_advertisements: NetStatValue;
    redirects: NetStatValue;
    router_renumberings: NetStatValue;
  };
  tcp_ipv4: {
    active_opens: number;
    passive_opens: number;
    failed_connection_attempts: number;
    reset_connections: number;
    current_connections: number;
    segments_received: number;
    segments_sent: number;
    segments_retransmitted: number;
  };
  tcp_ipv6: {
    active_opens: number;
    passive_opens: number;
    failed_connection_attempts: number;
    reset_connections: number;
    current_connections: number;
    segments_received: number;
    segments_sent: number;
    segments_retransmitted: number;
  };
  udp_ipv4: {
    datagrams_received: number;
    no_ports: number;
    receive_errors: number;
    datagrams_sent: number;
  };
  udp_ipv6: {
    datagrams_received: number;
    no_ports: number;
    receive_errors: number;
    datagrams_sent: number;
  };
};

export type NetStatKeys = keyof NetStat;
export type NetStatChildKeys = keyof NetStat[NetStatKeys];

export type Connection = {
  connection_type: string;
  local_address: string;
  foreign_address: string;
  state: string;
  pid: string;
};

export type NetConnections = Record<string, Connection[]>;

export type Process = {
  image_name: string;
  pid: number;
  session_name: string;
  session_number: string;
};
