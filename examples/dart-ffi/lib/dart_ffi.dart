import 'dart:convert';
import 'dart:ffi';

import 'package:dart_ffi/src/prisma_bindings.dart';
import 'package:ffi/ffi.dart';

calculate() {
  final library = DynamicLibrary.open('../../target/debug/libprisma.dylib');

  try {
    final bindings = PrismaBindings(library);
    final version = bindings.prisma_version().cast<Utf8>().toDartString();

    print(version);

    final schema = '''
generator client {
provider = "prisma-client-js"
output   = "../node_modules/@generated/photon"
}

datasource db {
provider = "sqlite"
url      = "file:dev.db"
}

model User {
id  String @id @default(cuid())
}
''';
    final formatOptions = {
      "textDocument": {"uri": 'file:/dev/null'},
      "options": {
        "tabSize": 2,
        "insertSpaces": true,
      }
    };
    final formated = bindings.prisma_schema_format(
      schema.toNativeUtf8().cast(),
      json.encode(formatOptions).toNativeUtf8().cast(),
    );

    print(formated.cast<Utf8>().toDartString());
  } finally {
    library.close();
  }
}
