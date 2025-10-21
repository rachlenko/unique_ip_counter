# Changelog

### Changed
- **BREAKING**: Migrated IP address storage from `String` to native types (`Ipv4Addr`/`Ipv6Addr`)
  - IPv4 addresses now stored as `std::net::Ipv4Addr` (4 bytes) instead of `String` (~36-39 bytes)
  - IPv6 addresses now stored as `std::net::Ipv6Addr` (16 bytes) instead of `String` (~63+ bytes)

### Performance Improvements
- **Memory efficiency**: Reduced memory usage by ~9-10x for IPv4 addresses (36-40 bytes → 4 bytes)
- **Memory efficiency**: Reduced memory usage by ~4x for IPv6 addresses (~63+ bytes → 16 bytes)
- **Comparison speed**: IP comparisons now execute in single CPU instruction vs string comparison
- **Hashing performance**: Integer-based hashing significantly faster than string hashing
- **Cache utilization**: Improved CPU cache locality due to compact data representation

### Technical Details

#### String Storage (Previous)
- **IPv4 Example**: `"192.168.1.1"`
  - Minimum: 7 bytes (`"0.0.0.0"`)
  - Maximum: 15 bytes (`"255.255.255.255"`)
  - Average: ~12-13 bytes
  - String overhead: 24 bytes (pointer, length, capacity)
  - **Total: ~36-39 bytes per address**

### Read document [SDD](SystemDesignDocument.md) as alternative better plan 
