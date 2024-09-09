// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'std_response.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

StdResponse _$StdResponseFromJson(Map<String, dynamic> json) => StdResponse(
      json['success'] as bool,
      json['msg'] as String?,
      json['data'],
    );

Map<String, dynamic> _$StdResponseToJson(StdResponse instance) =>
    <String, dynamic>{
      'success': instance.success,
      'msg': instance.msg,
      'data': instance.data,
    };
