from setuptools import setup

requires = [
        'pyramid',
        'bcrypt',
        'pyramid_jinja2',
        'ffmpeg-python'
]
setup(name='app',
        install_requires=requires,
)

