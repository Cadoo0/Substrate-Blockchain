from scalecodec.type_registry import load_type_registry_file
from substrateinterface import SubstrateInterface, Keypair, SubstrateRequestException
import click
from os import getcwd, path
import json

custom_type_registry = load_type_registry_file('custom-types.json')

substrate = SubstrateInterface(
    url="ws://127.0.0.1:9944",
    address_type=42,
    type_registry_preset='substrate-node-template',
    type_registry=custom_type_registry
)


def get_mnemonic(user):
    current_path = getcwd()
    clients_path = current_path + '/clients/'
    client_path = clients_path + '/' + user
    with open(client_path + '/mnemonic.txt', 'r') as mnemonic:
        return mnemonic.read()


def get_keypair(user):
    return Keypair.create_from_mnemonic(get_mnemonic(user))


@click.group()
def cli():
    pass


@click.command()
@click.option('--user', help='The user making the request', prompt='User')
@click.option('--from', 'sender', help='The library sending the book', prompt='From')
@click.option('--to', help='The library receiving the book', prompt='To')
@click.option('--book', help='The book in question', prompt='Book')
@click.option('--transporter', help='The transporter who is transporting the book', prompt='Transporter')
def add_book_transaction(user, sender, to, book, transporter):
    keypair = get_keypair(user)
    call = substrate.compose_call(
        call_module='TemplateModule',
        call_function='add_transaction',
        call_params={
            'from': sender,
            'to': to,
            'book_id': book,
            'transporter_id': transporter
        }
    )
    extrinsic = substrate.create_signed_extrinsic(call=call, keypair=keypair)
    try:
        substrate.submit_extrinsic(extrinsic, wait_for_inclusion=True)
        print("Boek transactie toegevoegd")
    except SubstrateRequestException as e:
        print("Error: {}".format(e))


@click.command()
@click.option('--user', help='The user making the request', prompt='User')
@click.option('--id', help='A unique id that identifies this book', prompt='Id')
@click.option('--isbn', help='A unique id that identifies this book', prompt='Isbn')
@click.option('--name', help='The book\'s name', prompt='Name')
def add_book(user, id, isbn, name):
    keypair = get_keypair(user)
    call = substrate.compose_call(
        call_module='TemplateModule',
        call_function='add_book',
        call_params={
            'id': id,
            'isbn': isbn,
            'name': name,
            'owner': None
        }
    )
    extrinsic = substrate.create_signed_extrinsic(call=call, keypair=keypair)
    try:
        substrate.submit_extrinsic(extrinsic, wait_for_inclusion=True)
        print("Boek toegevoegd")
    except SubstrateRequestException as e:
        print("Error: {}".format(e))


@click.command()
@click.option('--user', help='The user making the request', prompt='User')
@click.option('--id', help='A unique id that identifies this library', prompt='Id')
@click.option('--name', help='The library\'s name', prompt='Name')
def add_library(user, id, name):
    keypair = get_keypair(user)
    call = substrate.compose_call(
        call_module='TemplateModule',
        call_function='add_library',
        call_params={
            'id': id,
            'name': name,
            'books': []
        }
    )
    extrinsic = substrate.create_signed_extrinsic(call=call, keypair=keypair)
    try:
        substrate.submit_extrinsic(extrinsic, wait_for_inclusion=True)
        print("Bibliotheek toegevoegd")
    except SubstrateRequestException as e:
        print("Error: {}".format(e))


@click.command()
@click.option('--id', help='The id of the book you wish to retrieve', prompt='Id')
def get_book(id):
    book_info = substrate.get_runtime_state(
        module='TemplateModule',
        storage_function='Books',
        params=[id]
    ).get('result')

    click.echo(json.dumps(book_info, indent=4, sort_keys=True))


@click.command()
@click.option('--id', help='The id of the library you wish to retrieve', prompt='Id')
def get_library(id):
    library_info = substrate.get_runtime_state(
        module='TemplateModule',
        storage_function='Libraries',
        params=[id]
    ).get('result')

    click.echo(json.dumps(library_info, indent=4, sort_keys=True))


@click.command()
def get_metadata():
    click.echo(json.dumps(substrate.get_metadata_storage_functions(), indent=4, sort_keys=True))


cli.add_command(add_book_transaction)
cli.add_command(add_book)
cli.add_command(add_library)
cli.add_command(get_book)
cli.add_command(get_library)


if __name__ == '__main__':
    cli()

