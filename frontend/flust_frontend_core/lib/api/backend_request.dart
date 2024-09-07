import 'package:flust_frontend_core/api/backend_api.dart';
import 'package:flust_frontend_core/api/method.dart';
import 'package:flust_frontend_core/api/std_response.dart';

class BackendRequest {
  final HttpMethod method;
  final String endpoint;
  final String? body;
  final Map<String, String>? queryParams;

  const BackendRequest({required this.method, required this.endpoint, this.body, this.queryParams});

  Future<StdResponse?> execute() {
    return BackendApi.request(method, endpoint, body: body, queryParams: queryParams);
  }
}