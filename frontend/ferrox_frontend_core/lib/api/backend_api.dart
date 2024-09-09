import 'dart:convert';

import 'package:ferrox_frontend_core/api/method.dart';
import 'package:ferrox_frontend_core/api/std_response.dart';
import 'package:ferrox_frontend_core/api/url/url_stub.dart'
  if (dart.library.io) 'package:ferrox_frontend_core/api/url/url_io.dart'
  if (dart.library.html) 'package:ferrox_frontend_core/api/url/url_web.dart';
import 'package:http/http.dart' as http;


class BackendApi {
  static late String baseUrl;
  static late Function() onUnauthorized;

  static void init(int devPort, Function() onUnauthorized) {
    baseUrl = getBaseUrl(devPort);
  }

  static Future<StdResponse?> request(HttpMethod method, String endpoint, {String? body, Map<String, String>? queryParams}) async {
    var uri = Uri.parse('$baseUrl/$endpoint');

    if (queryParams != null) {
      uri.queryParameters.addAll(queryParams);
    }

    var request = http.Request(method.name, uri);
    if (body != null) {
      request.body = body;
    }

    if (method != HttpMethod.get) {
      request.headers['Content-Type'] = 'application/json';
    }

    http.StreamedResponse response;
    try {
      response = await request.send();
    } on http.ClientException {
      return await Future.delayed(const Duration(seconds: 1), () async {
        return await BackendApi.request(method, endpoint,
            body: body, queryParams: queryParams);
      });
    }

    if (response.statusCode == 401) {
      onUnauthorized();
      return null;
    }

    var text = await response.stream.bytesToString();

    return StdResponse.fromJson(jsonDecode(text));
  }
}