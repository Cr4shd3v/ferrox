import 'package:json_annotation/json_annotation.dart';

part 'std_response.g.dart';

@JsonSerializable()
class StdResponse {
  final bool success;
  final String? msg;
  final dynamic data;

  StdResponse(this.success, this.msg, this.data);

  factory StdResponse.fromJson(Map<String, dynamic> json) =>
      _$StdResponseFromJson(json);

  T? parse<T>(T Function(Map<String, dynamic> json) func) {
    if (data == null) {
      return null;
    } else {
      return func(data!);
    }
  }
}