namespace HttpGenerator.Tests.Resources;

public static class EmbeddedResources
{
    private static readonly Type Type = typeof(EmbeddedResources);

    private static Stream GetStream(string name)
        => Type.Assembly.GetManifestResourceStream(Type, name)!;

    public static string SwaggerPetstoreJsonV2
    {
        get
        {
            using var stream = GetStream("V2.SwaggerPetstore.json");
            using var reader = new StreamReader(stream);
            return reader.ReadToEnd();
        }
    }

    public static string SwaggerPetstoreJsonV3
    {
        get
        {
            using var stream = GetStream("V3.SwaggerPetstore.json");
            using var reader = new StreamReader(stream);
            return reader.ReadToEnd();
        }
    }

    public static string SwaggerPetstoreYamlV2
    {
        get
        {
            using var stream = GetStream("V2.SwaggerPetstore.yaml");
            using var reader = new StreamReader(stream);
            return reader.ReadToEnd();
        }
    }

    public static string SwaggerPetstoreYamlV3
    {
        get
        {
            using var stream = GetStream("V3.SwaggerPetstore.yaml");
            using var reader = new StreamReader(stream);
            return reader.ReadToEnd();
        }
    }


    public static string SwaggerPetstoreJsonV2WithDifferentHeaders
    {
        get
        {
            using var stream = GetStream("V2.SwaggerPetstoreWithDifferentHeaders.json");
            using var reader = new StreamReader(stream);
            return reader.ReadToEnd();
        }
    }

    public static string SwaggerPetstoreJsonV3WithDifferentHeaders
    {
        get
        {
            using var stream = GetStream("V3.SwaggerPetstoreWithDifferentHeaders.json");
            using var reader = new StreamReader(stream);
            return reader.ReadToEnd();
        }
    }

    public static string SwaggerPetstoreYamlV2WithDifferentHeaders
    {
        get
        {
            using var stream = GetStream("V2.SwaggerPetstoreWithDifferentHeaders.yaml");
            using var reader = new StreamReader(stream);
            return reader.ReadToEnd();
        }
    }

    public static string SwaggerPetstoreYamlV3WithDifferentHeaders
    {
        get
        {
            using var stream = GetStream("V3.SwaggerPetstoreWithDifferentHeaders.yaml");
            using var reader = new StreamReader(stream);
            return reader.ReadToEnd();
        }
    }

    public static string GetStringFromEmbeddedResource(string name)
    {
        using var stream = GetStream(name);
        using var reader = new StreamReader(stream);
        return reader.ReadToEnd();
    }

    public static string GetSwaggerPetstore(Samples version)
    {
        return version switch
        {
            Samples.PetstoreJsonV2 => SwaggerPetstoreJsonV2,
            Samples.PetstoreYamlV2 => SwaggerPetstoreYamlV2,
            Samples.PetstoreYamlV3 => SwaggerPetstoreYamlV3,
            Samples.PetstoreJsonV2WithDifferentHeaders => SwaggerPetstoreJsonV2WithDifferentHeaders,
            Samples.PetstoreJsonV3WithDifferentHeaders => SwaggerPetstoreJsonV3WithDifferentHeaders,
            Samples.PetstoreYamlV2WithDifferentHeaders => SwaggerPetstoreYamlV2WithDifferentHeaders,
            Samples.PetstoreYamlV3WithDifferentHeaders => SwaggerPetstoreYamlV3WithDifferentHeaders,
            Samples.PetstoreJsonV3WithMultlineDescriptions => GetStringFromEmbeddedResource("V3.SwaggerPetstoreWithMultlineDescriptions.json"),
            Samples.PetstoreYamlV3WithMultlineDescriptions => GetStringFromEmbeddedResource("V3.SwaggerPetstoreWithMultlineDescriptions.yaml"),
            _ => SwaggerPetstoreJsonV3
        };
    }
}