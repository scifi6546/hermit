from setuptools import setup

requires = [
        'pyramid',
        'bcrypt',
        'pyramid_jinja2',
        'ffmpeg-python',
        'python-magic',
        'pyramid_jinja2'
]
setup(name='app',
        install_requires=requires,
)

