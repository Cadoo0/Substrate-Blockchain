import json

from scalecodec.type_registry import load_type_registry_file
from substrateinterface import SubstrateInterface, Keypair, SubstrateRequestException

custom_type_registry = load_type_registry_file('custom-types.json')

substrate = SubstrateInterface(
    url="ws://127.0.0.1:9944",
    address_type=42,
    type_registry_preset='substrate-node-template',
    type_registry=custom_type_registry
)

# print(json.dumps(substrate.get_metadata_storage_functions(), indent=4, sort_keys=True))

library_info = substrate.get_runtime_state(
    module='TemplateModule',
    storage_function='Libraries',
    params=['1']
).get('result')

print(library_info)