import 'dart:html' as html;

String getBaseUrl(int devPort) {
  return '${html.document.baseUri!}/api';
}