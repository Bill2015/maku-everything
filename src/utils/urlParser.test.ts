import { getNameAndExtFromPath, getYoutubeVideoId, stringNormalize } from './urlParser';

describe('getYoutubeVideoId', () => {
    it.each([
        // normal url
        {
            inputURL: 'https://www.youtube.com/watch?v=uWxGMzSdX0c',
            expectId: 'uWxGMzSdX0c',
        },
        {
            inputURL: 'https://www.youtube.com/watch?v=uWxGMzSdX0c&list=RDuWxGMzSdX0c&start_radio=1',
            expectId: 'uWxGMzSdX0c',
        },
        {
            inputURL: 'https://www.youtube.com/watch?v=uWxGMzSdX0c?list=RDuWxGMzSdX0c',
            expectId: 'uWxGMzSdX0c',
        },
        // shorts
        {
            inputURL: 'https://www.youtube.com/shorts/7IxS5Y2eMw0',
            expectId: '7IxS5Y2eMw0',
        },
        {
            inputURL: 'https://www.youtube.com/shorts/7IxS5Y2eMw0&list=RDuWxGMzSdX0c&',
            expectId: '7IxS5Y2eMw0',
        },
    ])('Should get "$expectId" youtube id when input Youtube URL: "$inputURL"', ({ inputURL, expectId }) => {
        // Arrange

        // Act
        const result = getYoutubeVideoId(inputURL);

        // Assert
        expect(result).toEqual(expectId);
    });

    it.each([
        // normal
        'https://wwwabc.youtube.com/watch?v=uWxGMzSdX0c',
        'https://www.youtube.com/watch?vaaaaa=uWxGMzSdX0c',
        'https://www.youtube.com/watch?v=',
        // shorts
        'https://www.youtube.com/longs/7IxS5Y2eMw0',
        'https://www.youtube.com/shorts/',
    ])('Should return null when input Youtube URL: "%s"', (val) => {
        // Arrange

        // Act
        const result = getYoutubeVideoId(val);

        // Assert
        expect(result).toBeNull();
    });
});

describe('getFileNameFromPath', () => {
    it.each([
        {
            inputPath: 'D:\\Repo\\maku-everything\\dataset\\test.png',
            expected:  ['test', 'png'],
        },
        {
            inputPath: 'D:/Repo/maku-everything/dataset/test.png',
            expected:  ['test', 'png'],
        },
        {
            inputPath: 'test.png',
            expected:  ['test', 'png'],
        },
        {
            inputPath: 'D:/Repo/maku-everything/dataset/test.ext1.ext2.ext3',
            expected:  ['test.ext1.ext2', 'ext3'],
        },
        {
            inputPath: 'D:/Repo/maku-everything/dataset/test',
            expected:  ['test', ''],
        },
        {
            inputPath: 'test',
            expected:  ['test', ''],
        },
    ])('Should get "$expected" result when input FilePath: "$inputPath"', ({ inputPath, expected }) => {
        // Arrange

        // Act
        const result = getNameAndExtFromPath(inputPath);

        // Assert
        expect(result).toEqual(expected);
    });
});

describe('stringNormalize', () => {
    it.each([
        {
            inputStr: "abc's test#s",
            expected: 'abcstests',
        },
        {
            inputStr: "abc's<test#s>",
            expected: 'abcstests',
        },
        {
            inputStr: "abc's 你好test#s",
            expected: 'abcs你好tests',
        },
    ])('Should get "$expected" result when input String: "$inputStr"', ({ inputStr, expected }) => {
        // Arrange

        // Act
        const result = stringNormalize(inputStr);

        // Assert
        expect(result).toEqual(expected);
    });
});
