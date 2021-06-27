import enum
import re
import os.path
import typing

from googleapiclient.discovery import build
from google_auth_oauthlib.flow import InstalledAppFlow
from google.auth.transport.requests import Request
from google.oauth2.credentials import Credentials

from pydantic import BaseModel

SCOPES = ['https://www.googleapis.com/auth/documents']
DOCUMENT_ID = '1Twa1ZafoRzKTiRVfQaLZkKvdkOTDEbksoWrOXrPh7oc'
REG = re.compile(r'\$\$(\w+) (\w{6})\$\$')


class Variables(str, enum.Enum):
    listing = 'Листинг'
    diagram = 'Диаграмма'
    table = 'Таблица'
    attachment = 'Приложение'


VARIABLES: dict[Variables, typing.Union[int, str]] = {v: 1 for v in Variables}
VARIABLES[Variables.attachment] = 'А'
service: typing.Any = None
tokens = {}


class Item(BaseModel):
    start: int
    end: int
    variable: str
    token: str


def read_paragraph_element(element) -> typing.Iterator[Item]:
    if 'textRun' in element:
        content = element['textRun'].get('content', '')
        for m in REG.finditer(content):
            variable, token = m.groups()
            yield Item(
                start=element['startIndex'] + m.start(),
                end=element['startIndex'] + m.end(),
                variable=variable,
                token=token,
            )


def read_strucutural_elements(elements) -> typing.Iterator[Item]:
    for value in elements:
        if 'paragraph' in value:
            elements = value.get('paragraph').get('elements')
            for elem in elements:
                yield from read_paragraph_element(elem)
        elif 'table' in value:
            table = value.get('table')
            for row in table.get('tableRows'):
                cells = row.get('tableCells')
                for cell in cells:
                    yield from read_strucutural_elements(cell.get('content'))
        elif 'tableOfContents' in value:
            toc = value.get('tableOfContents')
            yield from read_strucutural_elements(toc.get('content'))


def auth():
    creds = None
    if os.path.exists('token.json'):
        creds = Credentials.from_authorized_user_file('token.json', SCOPES)

    if not creds or not creds.valid:
        if creds and creds.expired and creds.refresh_token:
            creds.refresh(Request())
        else:
            flow = InstalledAppFlow.from_client_secrets_file('credentials.json', SCOPES)
            creds = flow.run_local_server(port=0)

        with open('token.json', 'w') as token:
            token.write(creds.to_json())

    assert creds, 'Auth failed'
    return creds


def delete(start, end):
    requests = [{'deleteContentRange': {'range': {'startIndex': start, 'endIndex': end}}}]
    return service.documents().batchUpdate(documentId=DOCUMENT_ID, body={'requests': requests}).execute()


def insert(start, text):
    requests = [{'insertText': {'location': {'index': start}, 'text': text}}]
    return service.documents().batchUpdate(documentId=DOCUMENT_ID, body={'requests': requests}).execute()


def replace_one(item: Item):
    if not (t := [v for v in VARIABLES if v.name.lower() == item.variable.lower()]):
        raise NameError('incorrect variable')
    v = t[0]

    text = tokens.get(item.token, f'{v.value} {VARIABLES[v]}')
    delete(item.start, item.end)
    insert(item.start, text)

    if item.token not in tokens:
        tokens[item.token] = text
        if isinstance(VARIABLES[v], int):
            VARIABLES[v] += 1
        else:
            VARIABLES[v] = chr(ord(VARIABLES[v]) + 1)

    tokens.pop('number', '')
    return True


def need_to_change() -> typing.Iterator[Item]:
    while True:
        document = service.documents().get(documentId=DOCUMENT_ID).execute()
        doc_content = document.get('body').get('content')
        try:
            item = min(
                read_strucutural_elements(doc_content),
                key=lambda t: t.start,
            )
            yield item
        except ValueError:
            return


def main():
    global service
    creds = auth()
    service = build('docs', 'v1', credentials=creds)

    for item in need_to_change():
        print(item)
        replace_one(item)


if __name__ == '__main__':
    main()
