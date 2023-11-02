import 'dart:ffi';

import 'package:dart_ffi/src/prisma_bindings.dart';
import 'package:ffi/ffi.dart';

calculate() {
  final library = DynamicLibrary.open('../../target/debug/libprisma.dylib');

  try {
    final bindings = PrismaBindings(library);
    final version =
        bindings.get_prisma_semantic_version().cast<Utf8>().toDartString();

    print(version);
  } finally {
    library.close();
  }
}
