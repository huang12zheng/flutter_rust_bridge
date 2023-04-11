# Methods

There is support for structs with methods. Both static methods, and non-static methods are supported.

## Example

```rust,noplayground
pub struct SumWith { pub x: u32 }

impl SumWith {
  pub fn sum(&self, y: u32) -> u32 { self.x + y }
  pub fn sum_static(x: u32, y: u32) -> u32 { x + y }
}
```

Becomes the following if no_use_bridge_in_method: false

```Dart
class SumWith {
  final FlutterRustBridgeExampleSingleBlockTest bridge;
  final int x;

  const SumWith({
    required this.bridge,
    required this.x,
  });

  Future<int> sum({required int y, dynamic hint}) => bridge.sumMethodSumWith(
        that: this,
        y: y,
      );

  static Future<int> sumStatic(
          {required FlutterRustBridgeExampleSingleBlockTest bridge, required int x, required int y, dynamic hint}) =>
      bridge.sumStaticStaticMethodSumWith(x: x, y: y, hint: hint);
}
```

Becomes the following if no_use_bridge_in_method: true

```Dart
import 'ffi.io.dart' if (dart.library.html) 'ffi.web.dart';

class SumWith {
  final int x;

  const SumWith({
    required this.x,
  });

  Future<int> sum({required int y, dynamic hint}) => api.sumMethodSumWith(
        that: this,
        y: y,
      );

  static Future<int> sumStatic({required int x, required int y, dynamic hint}) =>
      api.sumStaticStaticMethodSumWith(x: x, y: y, hint: hint);
}
```
> The api is static and its name is dependent on the rust_inputs
Remark: If you are curious about `Future`, have a look at [this](async_dart.md).

