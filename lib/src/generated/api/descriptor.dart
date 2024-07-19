// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import '../lib.dart';
import 'error.dart';
import 'key.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'types.dart';

// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `fmt`

class BdkDescriptor {
  final ExtendedDescriptor extendedDescriptor;
  final KeyMap keyMap;

  const BdkDescriptor({
    required this.extendedDescriptor,
    required this.keyMap,
  });

  Future<String> asString() =>
      core.instance.api.crateApiDescriptorBdkDescriptorAsString(
        that: this,
      );

  Future<String> asStringPrivate() =>
      core.instance.api.crateApiDescriptorBdkDescriptorAsStringPrivate(
        that: this,
      );

  Future<BigInt> maxSatisfactionWeight() =>
      core.instance.api.crateApiDescriptorBdkDescriptorMaxSatisfactionWeight(
        that: this,
      );

  // HINT: Make it `#[frb(sync)]` to let it become the default constructor of Dart class.
  static Future<BdkDescriptor> newInstance(
          {required String descriptor, required Network network}) =>
      core.instance.api.crateApiDescriptorBdkDescriptorNew(
          descriptor: descriptor, network: network);

  static Future<BdkDescriptor> newBip44(
          {required BdkDescriptorSecretKey secretKey,
          required KeychainKind keychainKind,
          required Network network}) =>
      core.instance.api.crateApiDescriptorBdkDescriptorNewBip44(
          secretKey: secretKey, keychainKind: keychainKind, network: network);

  static Future<BdkDescriptor> newBip44Public(
          {required BdkDescriptorPublicKey publicKey,
          required String fingerprint,
          required KeychainKind keychainKind,
          required Network network}) =>
      core.instance.api.crateApiDescriptorBdkDescriptorNewBip44Public(
          publicKey: publicKey,
          fingerprint: fingerprint,
          keychainKind: keychainKind,
          network: network);

  static Future<BdkDescriptor> newBip49(
          {required BdkDescriptorSecretKey secretKey,
          required KeychainKind keychainKind,
          required Network network}) =>
      core.instance.api.crateApiDescriptorBdkDescriptorNewBip49(
          secretKey: secretKey, keychainKind: keychainKind, network: network);

  static Future<BdkDescriptor> newBip49Public(
          {required BdkDescriptorPublicKey publicKey,
          required String fingerprint,
          required KeychainKind keychainKind,
          required Network network}) =>
      core.instance.api.crateApiDescriptorBdkDescriptorNewBip49Public(
          publicKey: publicKey,
          fingerprint: fingerprint,
          keychainKind: keychainKind,
          network: network);

  static Future<BdkDescriptor> newBip84(
          {required BdkDescriptorSecretKey secretKey,
          required KeychainKind keychainKind,
          required Network network}) =>
      core.instance.api.crateApiDescriptorBdkDescriptorNewBip84(
          secretKey: secretKey, keychainKind: keychainKind, network: network);

  static Future<BdkDescriptor> newBip84Public(
          {required BdkDescriptorPublicKey publicKey,
          required String fingerprint,
          required KeychainKind keychainKind,
          required Network network}) =>
      core.instance.api.crateApiDescriptorBdkDescriptorNewBip84Public(
          publicKey: publicKey,
          fingerprint: fingerprint,
          keychainKind: keychainKind,
          network: network);

  static Future<BdkDescriptor> newBip86(
          {required BdkDescriptorSecretKey secretKey,
          required KeychainKind keychainKind,
          required Network network}) =>
      core.instance.api.crateApiDescriptorBdkDescriptorNewBip86(
          secretKey: secretKey, keychainKind: keychainKind, network: network);

  static Future<BdkDescriptor> newBip86Public(
          {required BdkDescriptorPublicKey publicKey,
          required String fingerprint,
          required KeychainKind keychainKind,
          required Network network}) =>
      core.instance.api.crateApiDescriptorBdkDescriptorNewBip86Public(
          publicKey: publicKey,
          fingerprint: fingerprint,
          keychainKind: keychainKind,
          network: network);

  @override
  int get hashCode => extendedDescriptor.hashCode ^ keyMap.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is BdkDescriptor &&
          runtimeType == other.runtimeType &&
          extendedDescriptor == other.extendedDescriptor &&
          keyMap == other.keyMap;
}
