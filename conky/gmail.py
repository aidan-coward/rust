#!/usr/bin/env python

from password import password
from urllib.request import FancyURLopener

email = 'aidan.coward@gmail.com' # @gmail.com can be left out

url = 'https://%s:%s@mail.google.com/mail/feed/atom' % (email, password)

opener = FancyURLopener()
page = opener.open(url)

contents = page.read().decode('utf-8')

ifrom = contents.index('<fullcount>') + 11
ito   = contents.index('</fullcount>')

fullcount = contents[ifrom:ito]

if int(fullcount) != 0:
    print(fullcount + ' | ')
