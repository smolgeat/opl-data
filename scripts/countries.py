#!/usr/bin/env python3
#
# Helper script that transforms countries in various non-standard forms
# into the full names used by modules/opltypes/src/countries.rs.
#
# When run as a script, it fixes the "Country" columns.
#
# This is also set up to be importable as a library, so that importation
# scripts can use it directly.

import oplcsv
import sys
import os


# Map of substitutions.
COUNTRY_MAP = {
    # International Olympic Committee country codes.
    'AFG': 'Afghanistan',
    'ALB': 'Albania',
    'ALG': 'Algeria',
    'AND': 'Andorra',
    'ANG': 'Angola',
    'ANT': 'Antigua and Barbuda',
    'ARG': 'Argentina',
    'ARM': 'Armenia',
    'ARU': 'Aruba',
    'ASA': 'American Samoa',
    'AUS': 'Australia',
    'AUT': 'Austria',
    'AZE': 'Azerbaijan',
    'BAH': 'Bahamas',
    'BAN': 'Bangladesh',
    'BAR': 'Barbados',
    'BDI': 'Burundi',
    'BEL': 'Belgium',
    'BEN': 'Benin',
    'BER': 'Bermuda',
    'BHU': 'Bhutan',
    'BIH': 'Bosnia and Herzegovina',
    'BIZ': 'Belize',
    'BLR': 'Belarus',
    'BOL': 'Bolivia',
    'BOT': 'Botswana',
    'BRA': 'Brazil',
    'BRN': 'Bahrain',
    'BRU': 'Brunei',
    'BUL': 'Bulgaria',
    'BUR': 'Burkina Faso',
    'CAF': 'Central African Republic',
    'CAM': 'Cambodia',
    'CAN': 'Canada',
    'CAY': 'Cayman Islands',
    'CGO': 'Congo',
    'CHA': 'Chad',
    'CHI': 'Chile',
    'CHN': 'China',
    'CIV': 'Côte d’Ivoire',
    'CMR': 'Cameroon',
    'COD': 'Democratic Republic of the Congo',
    'COK': 'Cook Islands',
    'COL': 'Colombia',
    'COM': 'Comoros',
    'CPV': 'Cape Verde',
    'CRC': 'Costa Rica',
    'CRO': 'Croatia',
    'CUB': 'Cuba',
    'CYP': 'Cyprus',
    'CZE': 'Czechia',
    'DEN': 'Denmark',
    'DJI': 'Djibouti',
    'DMA': 'Dominica',
    'DOM': 'Dominican Republic',
    'ECU': 'Ecuador',
    'EGY': 'Egypt',
    'ERI': 'Eritrea',
    'ESA': 'El Salvador',
    'ESP': 'Spain',
    'EST': 'Estonia',
    'ETH': 'Ethiopia',
    'FIJ': 'Fiji',
    'FIN': 'Finland',
    'FRA': 'France',
    'FSM': 'Federated States of Micronesia',
    'GAB': 'Gabon',
    'GAM': 'The Gambia',
    'GBR': 'Britain',
    'GBS': 'Guinea-Bissau',
    'GEO': 'Georgia',
    'GEQ': 'Equatorial Guinea',
    'GER': 'Germany',
    'GHA': 'Ghana',
    'GRE': 'Greece',
    'GRN': 'Grenada',
    'GUA': 'Guatemala',
    'GUI': 'Guinea',
    'GUM': 'Guam',
    'GUY': 'Guyana',
    'HAI': 'Haiti',
    'HKG': 'Hong Kong',
    'HON': 'Honduras',
    'HUN': 'Hungary',
    'INA': 'Indonesia',
    'IND': 'India',
    'IRI': 'Iran',
    'IRL': 'Ireland',
    'IRQ': 'Iraq',
    'ISL': 'Iceland',
    'ISR': 'Israel',
    'ISV': 'US Virgin Islands',
    'ITA': 'Italy',
    'IVB': 'British Virgin Islands',
    'JAM': 'Jamaica',
    'JOR': 'Jordan',
    'JPN': 'Japan',
    'KAZ': 'Kazakhstan',
    'KEN': 'Kenya',
    'KGZ': 'Kyrgyzstan',
    'KIR': 'Kiribati',
    'KOR': 'South Korea',
    'KOS': 'Kosovo',
    'KSA': 'Saudi Arabia',
    'KUW': 'Kuwait',
    'LAO': 'Laos',
    'LAT': 'Latvia',
    'LBA': 'Libya',
    'LBN': 'Lebanon',
    'LBR': 'Liberia',
    'LCA': 'Saint Lucia',
    'LES': 'Lesotho',
    'LIE': 'Liechtenstein',
    'LTU': 'Lithuania',
    'LUX': 'Luxembourg',
    'MAD': 'Madagascar',
    'MAR': 'Morocco',
    'MAS': 'Malaysia',
    'MAW': 'Malawi',
    'MDA': 'Moldova',
    'MDV': 'Maldives',
    'MEX': 'Mexico',
    'MGL': 'Mongolia',
    'MHL': 'Marshall Islands',
    'MKD': 'Macedonia',
    'MLI': 'Mali',
    'MLT': 'Malta',
    'MNE': 'Montenegro',
    'MON': 'Monaco',
    'MOZ': 'Mozambique',
    'MRI': 'Mauritius',
    'MTN': 'Mauritania',
    'MYA': 'Myanmar',
    'NAM': 'Namibia',
    'NCA': 'Nicaragua',
    'NED': 'Netherlands',
    'NEP': 'Nepal',
    'NGR': 'Nigeria',
    'NIG': 'Niger',
    'NOR': 'Norway',
    'NRU': 'Nauru',
    'NZL': 'New Zealand',
    'OMA': 'Oman',
    'PAK': 'Pakistan',
    'PAN': 'Panama',
    'PAR': 'Paraguay',
    'PER': 'Peru',
    'PHI': 'Philippines',
    'PLE': 'Palestine',
    'PLW': 'Palau',
    'PNG': 'Papua New Guinea',
    'POL': 'Poland',
    'POR': 'Portugal',
    'PRK': 'North Korea',
    'PUR': 'Puerto Rico',
    'QAT': 'Qatar',
    'ROU': 'Romania',
    'RSA': 'South Africa',
    'RUS': 'Russia',
    'RWA': 'Rwanda',
    'SAM': 'Samoa',
    'SEN': 'Senegal',
    'SEY': 'Seychelles',
    'SGP': 'Singapore',
    'SKN': 'Saint Kitts and Nevis  ',
    'SLE': 'Sierra Leone',
    'SLO': 'Slovenia',
    'SMR': 'San Marino',
    'SOL': 'Solomon Islands',
    'SOM': 'Somalia',
    'SRB': 'Serbia',
    'SRI': 'Sri Lanka',
    'SSD': 'South Sudan',
    'STP': 'São Tomé and Príncipe',
    'SUD': 'Sudan',
    'SUI': 'Switzerland',
    'SUR': 'Suriname',
    'SVK': 'Slovakia',
    'SWE': 'Sweden',
    'SWZ': 'Swaziland',
    'SYR': 'Syria',
    'TAN': 'Tanzania',
    'TGA': 'Tonga',
    'THA': 'Thailand',
    'TJK': 'Tajikistan',
    'TKM': 'Turkmenistan',
    'TLS': 'East',
    'TOG': 'Togo',
    'TPE': 'Taiwan',
    'TTO': 'Trinidad and Tobago',
    'TUN': 'Tunisia',
    'TUR': 'Turkey',
    'TUV': 'Tuvalu',
    'UAE': 'United Arab Emirates',
    'UGA': 'Uganda',
    'UKR': 'Ukraine',
    'URU': 'Uruguay',
    'USA': 'USA',
    'UZB': 'Uzbekistan',
    'VAN': 'Vanuatu',
    'VEN': 'Venezuela',
    'VIE': 'Vietnam',
    'VIN': 'Saint Vincent and the Grenadines',
    'YEM': 'Yemen',
    'ZAM': 'Zambia',
    'ZIM': 'Zimbabwe'
}


def standardize_country_csv(csv):
    '''Standardizes the Country column.
       Returns true iff something was changed.'''
    global COUNTRY_MAP

    if 'Country' not in csv.fieldnames:
        return False
    idx = csv.index('Country')

    changed = False
    for row in csv.rows:
        country = row[idx]
        if country in COUNTRY_MAP:
            row[idx] = COUNTRY_MAP[country]
            changed = True

    return changed


def standardize_country_filename(filename):
    csv = oplcsv.Csv(filename)
    if standardize_country_csv(csv):
        csv.write_filename(filename)


if __name__ == '__main__':
    if len(sys.argv) > 1:
        standardize_country_filename(sys.argv[1])
    else:
        for dirname, subdirs, files in os.walk(os.getcwd()):
            if "meet-data" in subdirs:
                subdirs[:] = ['meet-data']
            if 'entries.csv' in files:
                filepath = dirname + os.sep + 'entries.csv'
                standardize_country_filename(filepath)
