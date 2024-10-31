import 'dart:html' as html;

import 'package:flutter/foundation.dart';

String getBaseUrl(int devPort) {
  if (kDebugMode) {
    return 'http://127.0.0.1:$devPort/api';
  } else {
    return '${html.document.baseUri!}/api';
  }
}