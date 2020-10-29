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

keypair = Keypair.create_from_mnemonic('swift clown ride welcome cattle sunset cactus cinnamon talent april rose fever')

call = substrate.compose_call(
    call_module='TemplateModule',
    call_function='claim_book',
    call_params={
        'library_id': '1',
        'book_id': '1'
    }
)

extrinsic = substrate.create_signed_extrinsic(call=call, keypair=keypair)

try:
    result = substrate.submit_extrinsic(extrinsic, wait_for_inclusion=True)
    print("Boek geclaimd")

except SubstrateRequestException as e:
    print("Failed to send: {}".format(e))