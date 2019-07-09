from setuptools import setup

requires = [
        'pyramid',
        'bcrypt',
        'python-magic',
        'pyramid_jinja2'
]
setup(name='app',
        install_requires=requires,
)

